# Change Log

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] -- 2019-10-18

## [1.1.0] -- 2019-10-18
### Added
- Support for atomic exchange on stable linux

## [1.0.0] -- 2019-10-17
### Changed
 - Updated dependencies
 - Use the RENAME_EXCHANGE constant provided by newer libc version
 - Removed error-chain dependency, instead use custom error type
 - Remove cfg_if dependency, replace with cfg_attr
 - Remove clap, instead use custom command line parsing
 - Update to 2018 edition

## [0.2.1] -- 2017-09-10
### Added
 - Link to docs in README
 - Full usage message in README

## [0.2.0] -- 2017-09-10
### Added
 - Linux support on nightly
 - Print new line at end of error messages
 - Add non-atomic generic fallback
 - Add flag to program to call non-atomic fallback 
### Changed
 - Run tests on non-atomic versions

## [0.1.0] -- 2017-09-02
### Added
 - License
 - Readme
 - This changelog
 - Windows implementation
