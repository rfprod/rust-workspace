#!/bin/bash

source tools/shell/utils/colors.sh ''
source tools/shell/utils/print-utils.sh ''

##
# Prints the script usage instuctions.
##
printUsage() {
  printInfoTitle "<< ${0} usage >>"
  printUsageTip "bash tools/shell/install.sh" "print help"
  printUsageTip "bash tools/shell/install.sh all" "install global npm dependencies, and shellcheck on linux"
  printUsageTip "bash tools/shell/install.sh all osx" "install global npm dependencies, and shellcheck on (osx)"
  printUsageTip "bash tools/shell/install.sh global" "install global dependencies on Linux"
  printUsageTip "bash tools/shell/install.sh global osx" "install global dependencies on OSX"
  printUsageTip "bash tools/shell/install.sh shellcheck" "install shellcheck on linux"
  printUsageTip "bash tools/shell/install.sh shellcheck osx" "install shellcheck on osx"
  printGap
}

##
# Installs global dependencies.
##
installGlobalDependencies() {
  if [ "$1" = "osx" ]; then
    installGlobalDependenciesLinux
  else
    installGlobalDependenciesOsx
  fi
}

##
# Installg global dependencies on Linux.
# It is assumed that NodeJS is installed.
##
installGlobalDependenciesLinux() {
  printInfoTitle "<< Intalling global NPM dependencies >>"
  printGap

  sudo npm install -g commitizen@latest cz-conventional-changelog@latest npm-check-updates@latest || exit 1
}

##
# Installg global dependencies on OSX.
# It is assumed that Python is installed.
##
installGlobalDependenciesOsx() {
  printInfoTitle "<< Intalling global Python dependencies >>"
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
  installGlobalDependencies "$2"
  installShellcheck "$2"
elif [ "$1" = "global" ]; then
  installGlobalDependencies "$2"
elif [ "$1" = "shellcheck" ]; then
  installShellcheck "$2"
else
  printUsage
  exit 1
fi
