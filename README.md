# Todors

TODO app, mainly for a practical learning experience of Rust

[![Deployment](https://github.com/meysam81/todors/actions/workflows/deploy-fly.yml/badge.svg)](https://todors.fly.dev/docs/)
[![Code Size](https://img.shields.io/github/languages/code-size/meysam81/todors)](https://github.com/meysam81/todors)
[![Repo Size](https://img.shields.io/github/repo-size/meysam81/todors)](https://github.com/meysam81/todors)
[![Language Count](https://img.shields.io/github/languages/count/meysam81/todors)](https://github.com/meysam81/todors)
[![Commit Intervals](https://img.shields.io/github/commit-activity/m/meysam81/todors)](https://github.com/meysam81/todors/commits)
[![Last Release](https://img.shields.io/github/release-date/meysam81/todors?label=last%20release)](https://github.com/meysam81/todors/releases)
[![GitHub Stars](https://img.shields.io/github/stars/meysam81/todors?label=GitHub%20stars)](https://github.com/meysam81/todors/stargazers)
[![GitHub Release Downloads](https://img.shields.io/github/downloads/meysam81/todors/total?label=GitHub%20Release%20Downloads)](https://github.com/meysam81/todors/releases)
[![Cargo Crate](https://img.shields.io/crates/v/todors)](https://crates.io/crates/todors)
[![Crate Download](https://img.shields.io/crates/d/todors?label=crate%20download)](https://crates.io/crates/todors)
[![Docker pulls](https://img.shields.io/docker/pulls/meysam81/todors?label=Docker%20pulls)](https://hub.docker.com/r/meysam81/todors)
[![Docker Image](https://img.shields.io/docker/image-size/meysam81/todors?label=Docker%20image)](https://hub.docker.com/r/meysam81/todors)
[![License](https://img.shields.io/github/license/meysam81/todors)](https://github.com/meysam81/todors)
[![Lines of Code](https://img.shields.io/tokei/lines/github/meysam81/todors?label=lines%20of%20code)](https://github.com/meysam81/todors)

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Todors](#todors)
  - [Installation](#installation)
    - [Cargo](#cargo)
    - [Download binary](#download-binary)
    - [Docker](#docker)
  - [Usage](#usage)
    - [REST API doc](#rest-api-doc)
      - [Online](#online)
      - [Local](#local)
        - [Run HTTP server](#run-http-server)
        - [Visit URL](#visit-url)
    - [gRPC API doc](#grpc-api-doc)
      - [Look at the proto files](#look-at-the-proto-files)
      - [Run gRPC server](#run-grpc-server)
  - [Help](#help)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Installation

### Cargo

```bash
cargo install todors
```

### Download binary

You can also head over to the
[GitHub release page](https://github.com/meysam81/todors/releases/latest) and download the
binary for your platform.

### Docker
  
```bash
docker pull meysam81/todors
# or
docker pull ghcr.io/meysam81/todors
```

## Usage

The usage is as follows:

```bash
todors serve grpc -p 50051 -H 127.0.0.1
todors serve http -p 8000 -H 127.0.0.1
# Both port & host are optional, but ipv6 can also be used
todors serve http -H ::1

todors create "My first todo"
todors list
todors update 1 --title "My first todo updated"
todors update 1 --done
todors update 1 --undone
todors delete 1

todors completion bash | sudo tee /etc/bash_completion.d/todors
```

### REST API doc

#### Online

<https://todors.fly.dev/docs/>

#### Local

##### Run HTTP server

```bash
todors serve http
```

##### Visit URL

<http://localhost:8080/docs/>

### gRPC API doc

#### Look at the proto files

[proto directory](./proto)

#### Run gRPC server

```bash
todors serve grpc
# exposed at localhost:50051
```

## Help

```bash
Usage: todors <COMMAND>
Commands:
  serve       Serve either the gRPC or REST over HTTP server
  create      Create a new TODO with a title
  delete      Delete a TODO by ID
  list        List all TODOs
  update      Update a TODO by ID
  completion  Generate shell completion
  help        Print this message or the help of the given subcommand(s)
Options:
  -h, --help     Print help
  -V, --version  Print version
```
