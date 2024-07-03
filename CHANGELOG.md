# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚀 Features

- Add github-actions to dependabot

### ⚙️ Miscellaneous Tasks

- Update Cargo.lock and format sources
- *(CI)* Cargo-upgrade on schedule
- Update cargo dependencies

## [0.10.6] - 2023-09-18

### 🚀 Features

- *(ci)* Clear old caches
- *(ci)* Allow manual run for clearing cache
- *(ci)* Allow clearing caches with configurable threshold

### 🐛 Bug Fixes

- *(ci)* Update flyctl version
- *(ci)* Install missing depenency
- *(ci)* Parse comma as id separator
- *(ci)* Install dependency with the right name
- *(ci)* Exit on failed json parsing
- *(ci)* Replace double with single quotes in python script
- *(ci)* Pass in the right variable
- *(ci)* Print jq naked and as is
- *(ci)* Avoid duplicate deserialization
- *(ci)* Stringify values
- *(ci)* Run bash command to get the values
- *(ci)* Handle default inside the step

### 📚 Documentation

- Add docs.rs badge

### ⚙️ Miscellaneous Tasks

- Update dependencies ⬆️
- Bump version

## [0.10.4] - 2023-05-18

### Msrv

- V1.65

## [0.10.0] - 2023-05-13

### 🚀 Features

- Add unittests for core logic (#51)
- Add prometheus metrics endpoint ✨

## [0.9.1] - 2023-05-05

### 🚀 Features

- *(ci)* Measure and upload coverage (#49)

### ⚙️ Miscellaneous Tasks

- Place REST API logic in one block

## [0.9.0] - 2023-05-03

### 🚀 Features

- Add logger to http interface (#43)
- Add gRPC interface (#45)

### 🐛 Bug Fixes

- Pass the failing test

### ⚙️ Miscellaneous Tasks

- Place public API of http at the top

## [0.8.0] - 2023-04-28

### 🚀 Features

- Add batch TODO creation (#36)
- Add JSON pretty print to CLI interface (#37)

### 📚 Documentation

- Update deploy badge

## [0.7.0] - 2023-04-27

### 🚀 Features

- Add swagger api doc (#33)

## [0.6.0] - 2023-04-26

### 🚀 Features

- Update fly.io deploy config
- Apply pagination on list API (#30)
- Introduce hard limit for list API (#32)

### 🐛 Bug Fixes

- *(ci)* Install protoc before build (#12)
- *(ci)* Deploy to fly only on main (#31)
- Add proto and build.rs to the Cargo definition
- *(ci)* Install protoc for publishing crate

### 📚 Documentation

- Update badge href
- Update release link to the latest

### ⚙️ Miscellaneous Tasks

- Update fly.io deployment config
- Listify workflow names in deploy-fly

### Deploy

- Add docker target serve all (#13)

## [0.5.0] - 2023-04-22

### 🚀 Features

- Implement http server logic (#8)
- Group docker ci on pull request first (#9)

### ⚙️ Miscellaneous Tasks

- Rename test job

## [0.4.1] - 2023-04-20

### 🐛 Bug Fixes

- Address docker build failure (#7)

## [0.4.0] - 2023-04-20

### 🚀 Features

- *(ci)* Add multiple platform target (#6)

## [0.3.5] - 2023-04-19

### 🐛 Bug Fixes

- Modify Dockerfile path

## [0.3.4] - 2023-04-19

### 📚 Documentation

- Update link to release & remove unused deps

## [0.3.3] - 2023-04-19

### 📚 Documentation

- Add installation guide

## [0.3.2] - 2023-04-19

### 📚 Documentation

- Place instructions in README

## [0.3.1] - 2023-04-19

### 🐛 Bug Fixes

- Specify max 5 keywords in cargo crate definition (#5)

## [0.3.0] - 2023-04-19

### 🚀 Features

- *(cli)* Add shell completion
- *(cli)* Add get API

### 📚 Documentation

- *(cli)* Update to reflect the recent changes
- Move guides to the root

### Release

- Upgrade version to v0.3.0 (#3)

## [0.2.1] - 2023-04-19

### 🚀 Features

- *(docs)* Add crates.io badges
- *(docs)* Add docker image size badge
- Upgrade version to v0.2.1

### 🐛 Bug Fixes

- *(ci)* Don't cancel concurrent docker image builders

### ⚙️ Miscellaneous Tasks

- Add pre-commit config

## [0.2.0] - 2023-04-19

### 🚀 Features

- *(cli)* Add `list` logic
- *(cli)* Complete the logic
- Update crate version to v0.2.0

### 🐛 Bug Fixes

- *(docs)* Update README badge url
- *(docs)* Update badge label

### 📚 Documentation

- *(cli)* Improve doc comments

### ⚙️ Miscellaneous Tasks

- Add docker build stable

## [0.1.0] - 2023-04-16

### 🐛 Bug Fixes

- *(ci)* Install mold for rust-doc
- *(ci)* Pass labels as string
- *(docs)* Update link to docker badge

### 📚 Documentation

- Update badges

### ⚙️ Miscellaneous Tasks

- Add rust-doc
- Try symbolic link for index.html
- Add docker build
- Add publish-crate

## [0.1.1] - 2023-04-16

### 🐛 Bug Fixes

- *(ci)* Dont fail fast
- *(ci)* Change build target
- *(ci)* Use toolchain for rust installation
- *(ci)* Add target using rustup
- *(ci)* Install toolchain for target
- *(ci)* Remove non-working builds
- *(ci)* Add permission to github token
- *(ci)* Include all builds
- *(ci)* Use glob for dir
- *(ci)* Rename duplicated key
- *(ci)* Dependency review ref
- *(ci)* Add debug
- *(ci)* Do file name manually

### ⚙️ Miscellaneous Tasks

- Add rust-lint
- Install mold
- Add rust-build
- Debug target dir
- Rust-clippy
- Dependabot
- Update deprecated
- Trivy scan
- Add sonar scan
- Dependency review (#1)
- Remove rust-lint
- Add checksum

### Debug

- *(ci)* List targets

<!-- generated by git-cliff -->
