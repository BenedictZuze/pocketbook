# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- v0.1.1 Automatic Health Checks on instances
- v0.1.1 Added system tray

### Bugs

- Outside closure of port running an instance will cause a mismatch of pid and port running instance in the master instance
  - scan or manually update existing instances in the master instance
  - on restart this seems to not matter but still a weird issue
