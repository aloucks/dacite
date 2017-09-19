# dacite-winit Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Items listed in "Changed" sub-sections are usually breaking changes. Any additional breaking changes
in other sub-sections are prefixed with "**BREAKING**" to increase visibility.


## [0.7.0] - 2017-09-19
This release contains breaking changes.

### Changed
 - Updated `bitflags` to version 1.0. `SurfaceCreateFlags` now uses associated consts. This
   shouldn't actually break any existing code, because there is only a single dummy value defined,
   which shouldn't be used anyway.
 - Updated `dacite` to 0.7.0


## [0.6.0] - 2017-07-08
This release contains breaking changes.

### Changed
 - Updated `dacite` to 0.6.0


## [0.5.0] - 2017-07-02
This release contains breaking changes.

### Changed
 - Updated `dacite` to 0.5.0


## [0.4.0] - 2017-06-25
This release contains breaking changes.

### Changed
 - Updated `dacite` to 0.4.0
 - Updated `winit` to 0.7.x.


## [0.3.0] - 2017-06-06
This release contains breaking changes.

### Changed
 - Updated `dacite` to 0.3.0


## [0.2.0] - 2017-06-05
This release contains breaking changes.

### Changed
 - Updated `dacite` to 0.2.0


## [0.1.0] - 2017-06-05
This is the initial release of dacite-winit.
