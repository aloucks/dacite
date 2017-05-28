#[macro_use]
extern crate clap;
extern crate fs_extra;
extern crate num_cpus;
extern crate tempdir;
extern crate toml;

use clap::{Arg, App};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;
use std::sync::mpsc;
use std::thread;
use tempdir::TempDir;

fn runner(id: usize, repo: PathBuf, jobs: mpsc::Receiver<Option<String>>, res: mpsc::Sender<(usize, Option<(String, process::Output)>)>) {
    res.send((id, None)).unwrap();

    loop {
        match jobs.recv().unwrap() {
            Some(feature) => {
                let tempdir = TempDir::new("dacite").unwrap();
                let mut sources = Vec::new();
                sources.push(repo.join("Cargo.toml"));
                if fs::metadata(repo.join("Cargo.lock")).is_ok() {
                    sources.push(repo.join("Cargo.lock"));
                }
                sources.push(repo.join("src"));
                fs_extra::copy_items(&sources, tempdir.path(), &fs_extra::dir::CopyOptions::new()).unwrap();

                let output = process::Command::new("cargo")
                    .arg("check")
                    .arg("--no-default-features")
                    .arg("--features")
                    .arg(&feature)
                    .stdin(process::Stdio::null())
                    .current_dir(tempdir.path())
                    .output()
                    .unwrap();

                res.send((id, Some((feature, output)))).unwrap();
            }

            None => { return; }
        }
    }
}

fn main() {
    let matches = App::new("dacite test-build")
        .arg(Arg::with_name("repo")
             .short("r")
             .long("repo")
             .value_name("PATH")
             .help("Path to the dacite repository [default: current directory]")
             .takes_value(true))
        .arg(Arg::with_name("j")
             .short("j")
             .value_name("NUM_JOBS")
             .help("Number of parallel builds [default: number of cores]")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .default_value("test-build.log")
             .value_name("FILE")
             .help("Output filename")
             .takes_value(true))
        .get_matches();

    let repo = matches.value_of("repo")
        .map(|r| fs::canonicalize(PathBuf::from(r)).unwrap())
        .unwrap_or(env::current_dir().unwrap());

    let mut num_jobs = value_t!(matches, "j", usize).unwrap_or(num_cpus::get());
    if num_jobs == 0 {
        num_jobs = 1;
    }

    let mut features: BTreeSet<_> = {
        let mut cargo_toml_file = fs::File::open(repo.join("Cargo.toml")).unwrap();
        let mut cargo_toml_cnt = String::new();
        cargo_toml_file.read_to_string(&mut cargo_toml_cnt).unwrap();
        let cargo_toml = cargo_toml_cnt.parse::<toml::Value>().unwrap();
        cargo_toml.as_table().unwrap()
            .get("features").unwrap().as_table().unwrap()
            .keys().cloned().collect()
    };

    features.insert(String::new());
    let num_features = features.len();

    let mut runners = Vec::new();
    let (res_send, res_recv) = mpsc::channel();
    for i in 0..num_jobs {
        let (job_send, job_recv) = mpsc::channel();
        let repo = repo.clone();
        let res_send = res_send.clone();
        let handle = thread::spawn(move || runner(i, repo, job_recv, res_send));
        runners.push((handle, job_send));
    }

    println!("Found {} features", num_features);
    println!("Running {} in parallel", num_jobs);
    println!("");
    let mut num_finished = 0usize;
    let mut features = features.iter();
    let mut log_file = fs::File::create(matches.value_of("output").unwrap()).unwrap();
    loop {
        let (id, job_res) = res_recv.recv().unwrap();
        if let Some((feature, output)) = job_res {
            if output.status.success() {
                println!("Feature \"{}\" succeeded", feature);
                writeln!(log_file, "Feature \"{}\" succeeded", feature).unwrap();;
            }
            else {
                println!("Feature \"{}\" failed", feature);
                writeln!(log_file, "\nFeature \"{}\" failed\n{}\n\n", feature, String::from_utf8(output.stderr).unwrap()).unwrap();
            }
            log_file.sync_all().unwrap();

            num_finished += 1;
        }

        if num_finished == num_features {
            break;
        }

        match features.next() {
            Some(feature) => {
                runners[id].1.send(Some(feature.clone())).unwrap();
            }

            None => { }
        }
    }

    for runner in runners {
        runner.1.send(None).unwrap();
        runner.0.join().unwrap();
    }
}
