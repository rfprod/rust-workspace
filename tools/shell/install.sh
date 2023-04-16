#!/bin/bash

source tools/shell/utils/config.sh
source tools/shell/utils/colors.sh ''
source tools/shell/utils/print-utils.sh ''

##
# Prints the script usage instuctions.
##
printUsage() {
  printInfoTitle "<< ${0} usage >>"
  printUsageTip "bash tools/shell/install.sh" "print help"
  printUsageTip "bash tools/shell/install.sh all" "install rustup, commitizen, and shellcheck on Linux"
  printUsageTip "bash tools/shell/install.sh all osx" "install rustup, commitizen, and shellcheck on OSX"
  printUsageTip "bash tools/shell/install.sh rustup" "install rustup on Linux or OSX"
  printUsageTip "bash tools/shell/install.sh commitizen" "install commitizen on Linux"
  printUsageTip "bash tools/shell/install.sh commitizen osx" "install commitizen on OSX"
  printUsageTip "bash tools/shell/install.sh shellcheck" "install shellcheck on Linux"
  printUsageTip "bash tools/shell/install.sh shellcheck osx" "install shellcheck on OSX"
  printUsageTip "bash tools/shell/install.sh dvc" "install DVC on Linux"
  printUsageTip "bash tools/shell/install.sh dvc osx" "install DVC on OSX"
  printGap
}

##
# Install rustup.
##
installRustup() {
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
}

##
# Install commitizen, cz-conventional-changelog, and npm-check-updates on Linux.
# It is assumed that NodeJS is installed.
##
installCommitizenLinux() {
  printInfoTitle "<< Intalling commitizen, cz-conventional-changelog, and npm-check-updates via NPM globaly >>"
  printGap

  sudo npm install -g commitizen@latest cz-conventional-changelog@latest npm-check-updates@latest || exit 1
}

##
# Install commitizen on OSX.
# It is assumed that Python is installed.
##
installCommitizenOsx() {
  printInfoTitle "<< Intalling commitizen via pypi globally >>"
  printGap

  brew install commitizen || exit 1
}

##
# Install global dependencies.
# Ref: https://commitizen.github.io/cz-cli/
##
installCommitizen() {
  if [ "$1" = "osx" ]; then
    installCommitizenLinux
  else
    installCommitizenOsx
  fi
}

##
# Install shellcheck on Linux.
##
installShellcheckLinux() {
  printInfoTitle "<< Installing shellcheck on Linux >>"
  printGap

  sudo apt -y install shellcheck
}

##
# Install shellcheck on OSX.
##
installShellcheckOsx() {
  printInfoTitle "<< Installing shellcheck on OSX >>"
  printGap

  brew install shellcheck
}

##
# Install shellcheck.
# Ref: https://www.shellcheck.net/
##
installShellcheck() {
  if [ "$1" = "osx" ]; then
    installShellcheckOsx
  else
    installShellcheckLinux
  fi
}

##
# Install DVC on Linux.
# Ref: https://dvc.org/doc/install/linux
##
installDvcLinux() {
  sudo wget https://dvc.org/deb/dvc.list -O /etc/apt/sources.list.d/dvc.list
  wget -qO - https://dvc.org/deb/iterative.asc | gpg --dearmor >packages.iterative.gpg
  sudo install -o root -g root -m 644 packages.iterative.gpg /etc/apt/trusted.gpg.d/
  rm -f packages.iterative.gpg
  sudo apt update
  sudo apt install dvc
}

##
# Install DVC on OSX.
# Ref: https://dvc.org/doc/install/macos
##
installDvcOsx() {
  brew install dvc
}

##
# Install DVC.
# Ref: https://dvc.org/doc/install
##
installDvc() {
  if [ "$1" = "osx" ]; then
    installDvcOsx
  else
    installDvcLinux
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
  installDvc "$2"
elif [ "$1" = "commitizen" ]; then
  installCommitizen "$2"
elif [ "$1" = "shellcheck" ]; then
  installShellcheck "$2"
elif [ "$1" = "dvc" ]; then
  installDvc "$2"
else
  printUsage
  exit 1
fi
