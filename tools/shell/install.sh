#!/bin/bash

source tools/shell/utils/colors.sh ''
source tools/shell/utils/print-utils.sh ''

##
# Prints the script usage instuctions.
##
printUsage() {
  printInfoTitle "<< ${0} usage >>"
  printUsageTip "bash tools/shell/install.sh" "print help"
  printUsageTip "bash tools/shell/install.sh all" "install rustup, commitizen, and shellcheck on linux"
  printUsageTip "bash tools/shell/install.sh all osx" "install rustup, commitizen, and shellcheck on OSX"
  printUsageTip "bash tools/shell/install.sh rustup" "install rustup on Linux or OSX"
  printUsageTip "bash tools/shell/install.sh commitizen" "install commitizen on Linux"
  printUsageTip "bash tools/shell/install.sh commitizen osx" "install commitizen on OSX"
  printUsageTip "bash tools/shell/install.sh shellcheck" "install shellcheck on linux"
  printUsageTip "bash tools/shell/install.sh shellcheck osx" "install shellcheck on OSX"
  printGap
}

##
# Installs rustup.
##
installRustup() {
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
}

##
# Installs global dependencies.
##
installCommitizen() {
  if [ "$1" = "osx" ]; then
    installCommitizenLinux
  else
    installCommitizenOsx
  fi
}

##
# Installs commitizen, cz-conventional-changelog, and npm-check-updates on Linux.
# It is assumed that NodeJS is installed.
##
installCommitizenLinux() {
  printInfoTitle "<< Intalling commitizen, cz-conventional-changelog, and npm-check-updates via NPM globaly >>"
  printGap

  sudo npm install -g commitizen@latest cz-conventional-changelog@latest npm-check-updates@latest || exit 1
}

##
# Installs commitizen on OSX.
# It is assumed that Python is installed.
##
installCommitizenOsx() {
  printInfoTitle "<< Intalling commitizen via pypi globally >>"
  printGap

  brew install commitizen || exit 1
}

##
# Installs shellcheck on Linux.
##
installShellcheckLinux() {
  printInfoTitle "<< Installing shellcheck on Linux >>"
  printGap

  sudo apt -y install shellcheck
}

##
# Installs shellcheck on OSX.
##
installShellcheckOsx() {
  printInfoTitle "<< Installing shellcheck on OSX >>"
  printGap

  brew install shellcheck
}

##
# Installs shellcheck.
##
installShellcheck() {
  if [ "$1" = "osx" ]; then
    installShellcheckOsx
  else
    installShellcheckLinux
  fi
}

##
# Dependencies installation control flow.
##
if [ "$1" = "?" ]; then
  printUsage
elif [ "$1" = "all" ]; then
  installRustup
  installShellcheck "$2"
  installCommitizen "$2"
elif [ "$1" = "commitizen" ]; then
  installCommitizen "$2"
elif [ "$1" = "shellcheck" ]; then
  installShellcheck "$2"
else
  printUsage
  exit 1
fi
