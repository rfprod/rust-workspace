# Rust workspace

Rust workspace with workflow automation.

[![Commitizen friendly](https://img.shields.io/badge/commitizen-friendly-brightgreen.svg)](http://commitizen.github.io/cz-cli/)

## Workflows

|                                                                              | Trigger                             | Badge                                                                                                                                                                                                      |
| ---------------------------------------------------------------------------- | ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [:information_source:](# 'Full testing, deliverables build and deployment.') | PR merge event (destination: trunk) | [![trunk](https://github.com/rfprod/rust-workspace/actions/workflows/trunk.yml/badge.svg)](https://github.com/rfprod/rust-workspace/actions/workflows/trunk.yml)                                           |
| [:information_source:](# 'Code ownership validation.')                       | Scheduled (weekly)                  | [![validate-codeowners](https://github.com/rfprod/rust-workspace/actions/workflows/validate-codeowners.yml/badge.svg)](https://github.com/rfprod/rust-workspace/actions/workflows/validate-codeowners.yml) |
| [:information_source:](# 'Quality gates: pull request validation.')          | PR open event (destination: trunk)  | [![validate-pr](https://github.com/rfprod/rust-workspace/actions/workflows/validate-pr.yml/badge.svg)](https://github.com/rfprod/rust-workspace/actions/workflows/validate-pr.yml)                         |

## Requirements

In order to run own copy of the project one must fulfill the following requirements.

### Supported operating systems

- :trophy: [Debian based Linux](https://en.wikipedia.org/wiki/List_of_Linux_distributions#Debian-based)
  - install all dependencies required to work with the project except NodeJS: rustup, commitizen, shellcheck
    ```bash
    bash tools/shell/install.sh all
    ```
  - see help for available options
    ```bash
    bash tools/shell/install.sh ?
    ```
- :ok: [OSX](https://en.wikipedia.org/wiki/MacOS)
  - install all dependencies required to work with the project except Python: rustup, commitizen, shellcheck
    ```bash
    bash tools/shell/install.sh all osx
    ```
  - see help for available options
    ```bash
    bash tools/shell/install.sh ?
    ```
- 🤷 [Windows](https://en.wikipedia.org/wiki/Microsoft_Windows)
  - install shellcheck
    ```powershell
    iwr -useb get.scoop.sh | iex
    scoop install shellcheck
    ```
  - recommended shell: [Git for Windows](https://gitforwindows.org/) > `Git BASH`
  - configure Git to use LF as a carriage return
    ```bash
    git config --global core.autocrlf false
    git config --global core.eol lf
    ```
  - one will have to figure out oneself how to install the `commitizen` package, the instructions for Linux will possibly work (see below)

### Integrated development environment

:trophy: [Visual Studio Code](https://code.visualstudio.com/) - recommended for all operating systems

### Core dependencies

- [Bash 5](https://www.gnu.org/software/bash/)
- [Python 3.6+](https://www.python.org/) - `for OSX to use the global commitizen installation`
- [NodeJS](https://nodejs.org/) - `for Linux to use the global commitizen installation`
- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/)

## Committing changes to the repo

### Linux

Using [commitizen cli](https://github.com/commitizen/cz-cli) is mandatory.

The commit message are validated during the premerge checks.

It is assumed that [Node.js](https://nodejs.org/) is installed.

Given the [NodeJS](https://nodejs.org/) is installed, and [commitizen cli is installed as a global dependency](https://github.com/commitizen/cz-cli#conventional-commit-messages-as-a-global-utility), the following command should be used to initiate the commit process

```bash
git cz
```

Alternatively, given there are no conflicts with other projects that use [the commitizen npm package](https://www.npmjs.com/package/commitizen), one could install commitizen globally via `pypi` like this

```bash
sudo pip3 install -U Commitizen
```

### OSX

Using [commitizen](https://pypi.org/project/commitizen/) is mandatory.

The commit message are validated during the premerge checks.

After installing the package as a global utility using the following command

```bash
brew install commitizen
```

one can use one of the following commands to initiate the commit process

```bash
cz commit
```

or

```bash
cz c
```

## General Tooling

This project was generated using [Cargo](https://doc.rust-lang.org/cargo/).

<p align="center"><img src="https://doc.rust-lang.org/cargo/images/Cargo-Logo-Small.png" width="450"></p>
<small>The Rust and Cargo logos (bitmap and vector) are owned by the Rust Foundation and distributed under the terms of the <a href="https://creativecommons.org/licenses/by/4.0/" target="_blank" rel="noreferer">Creative Commons Attribution license (CC-BY)</a>. The logos are used in compliance with the <a href="https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/" target="_blank" rel="noreferer">Rust Foundation Logo Policy and Media Guide</a>. No changes have been made to the original logos used by <a href="https://github.com/rfprod/rust-workspace" target="_blank" rel="noreferer">the project</a>. <a href="https://github.com/rfprod/rust-workspace" target="_blank" rel="noreferer">The project</a> was created for educational purposes.</small>

### What is Cargo

**Cargo is the [Rust](https://www.rust-lang.org/) [package manager](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-manager). Cargo downloads your Rust [package's](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io/), the Rust community [package registry](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-registry).**

### Quick Start & Documentation

- [The Rust language documentation](https://www.rust-lang.org/tools/install)

### Further help

```bash
rustup --help
```

```bash
cargo --help
```

## Technology Reference

### Workspace

- [rustup](https://rust-lang.github.io/rustup/)
- [Rust](https://doc.rust-lang.org/book/)
- [Cargo](https://doc.rust-lang.org/cargo)

### CI

- [GitHub Actions](https://github.com/features/actions)

### Development methodology

- [Trunk based development](https://trunkbaseddevelopment.com/)
