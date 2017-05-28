#!/bin/sh

SCRIPT=`realpath "${0}"`
SCRIPTPATH=`dirname "${SCRIPT}"`
PROJECTROOT=`realpath "${SCRIPTPATH}"/..`

cargo run --manifest-path "${PROJECTROOT}"/tools/test-build/Cargo.toml --release -- --repo "${PROJECTROOT}"
