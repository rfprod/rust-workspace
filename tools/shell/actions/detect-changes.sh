#!/bin/bash

########################################################################################
# Script: detect-changes.sh
# Purpose: Detect files changed between commits matching a regex pattern
# Usage: ./detect-changes.sh --pattern PATTERN [--compare-with COMPARE_WITH] [--verbose]
########################################################################################

set -euo pipefail

# Default values
PATTERN=""
COMPARE_WITH="origin/main"
VERBOSE=false

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print usage instructions.
print_usage() {
  cat << EOF
Usage: $0 --pattern PATTERN [OPTIONS]

Required:
  --pattern PATTERN              Regular expression pattern to match files

Optional:
  --compare-with COMPARE_WITH    Branch or commit to compare against
                                 (default: origin/main)
  --verbose                      Enable verbose output
  --help                         Display this help message

Examples:
  $0 --pattern '\.py$'
  $0 --pattern '^src/.*\.ts$' --compare-with develop
  $0 --pattern '(package\.json|\.lockfile)$' --compare-with HEAD~3

EOF
}

# Print an informational message.
log_info() {
  echo -e "${GREEN}[INFO]${NC} $*" >&2
}

# Print a warning message.
log_warn() {
  # shellcheck disable=SC2317
  echo -e "${YELLOW}[WARN]${NC} $*" >&2
}

# Print an error message.
log_error() {
  echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Print a debugging message.
log_verbose() {
  if [[ "$VERBOSE" == true ]]; then
    echo -e "${YELLOW}[DEBUG]${NC} $*" >&2
  fi
}

# Parse input arguments.
parse_arguments() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --pattern)
        if [[ -z "${2:-}" ]]; then
          log_error "Pattern argument requires a value"
          print_usage
          exit 1
        fi
        PATTERN="$2"
        shift 2
        ;;
      --compare-with)
        if [[ -z "${2:-}" ]]; then
          log_error "compare-with argument requires a value"
          print_usage
          exit 1
        fi
        COMPARE_WITH="$2"
        shift 2
        ;;
      --verbose)
        VERBOSE=true
        shift
        ;;
      --help)
        print_usage
        exit 0
        ;;
      *)
        log_error "Unknown option: $1"
        print_usage
        exit 1
        ;;
    esac
  done
}

# Validate input argument values.
validate_inputs() {
  if ! git rev-parse --git-dir > /dev/null 2>&1; then
    log_error "Not a git repository"
    exit 1
  fi

  if [[ -z "$PATTERN" ]]; then
    log_error "Pattern is required"
    print_usage
    exit 1
  fi

  local TEST_STRING
  TEST_STRING="test_file_pattern.txt"
  if ! grep -E "$PATTERN" /dev/null 2>&1; then
    if ! echo "$TEST_STRING" | grep -E "$PATTERN" 2>/dev/null && ! echo "" | grep -E "$PATTERN" 2>/dev/null; then
      local GREP_ERROR
      GREP_ERROR=$(echo "$TEST_STRING" | grep -E "$PATTERN" 2>&1 || true)
      if echo "$GREP_ERROR" | grep -qi "invalid\|error"; then
        log_error "Invalid regex pattern: $PATTERN"
        log_error "Grep error: $GREP_ERROR"
        exit 1
      fi
    fi
  fi

  log_verbose "Pattern: $PATTERN"
  log_verbose "Compare with: $COMPARE_WITH"
}

# Check whether a git ref exists.
check_ref_exists() {
  local REF
  REF="$1"
  if ! git rev-parse --verify "$REF" > /dev/null 2>&1; then
    log_error "Reference not found: $REF"
    log_info "Available branches:"
    git branch -a | sed 's/^/  /'
    exit 1
  fi
}

# Collect changed files between git refs.
get_changed_files() {
  local BASE_REF
  BASE_REF="$1"
  local HEAD_REF
  HEAD_REF="HEAD"

  log_verbose "Fetching diff between $BASE_REF and $HEAD_REF"

  git diff --name-only --diff-filter=AMDRC "$BASE_REF..$HEAD_REF"
}

# Filter files using a pattern provided via input arguments.
filter_files_by_pattern() {
  local REGX_PATTERN
  REGX_PATTERN="$1"

  local FILE
  while IFS= read -r FILE; do
    if [[ -z "$FILE" ]]; then
      continue
    fi

    log_verbose "Checking file: $FILE"

    if [[ "$FILE" =~ $REGX_PATTERN ]]; then
      echo "$FILE"
    fi
  done
}

# Entrypoint.
main() {
  parse_arguments "$@"
  validate_inputs

  log_info "Detecting changes matching pattern: $PATTERN"
  log_info "Comparing HEAD against: $COMPARE_WITH"

  check_ref_exists "$COMPARE_WITH"

  local CHANGED_FILES
  CHANGED_FILES=$(get_changed_files "$COMPARE_WITH" | filter_files_by_pattern "$PATTERN")

  if [[ -z "$CHANGED_FILES" ]]; then
    log_info "No matching files found"
    exit 0
  fi

  log_info "Found matching files:"
  echo "$CHANGED_FILES" | while IFS= read -r file; do
    echo "  - $file"
  done

  if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
    local FILES_JSON
    FILES_JSON=$(echo "$CHANGED_FILES" | jq -R -s -c 'split("\n")[:-1]')
    echo "changed_files=$FILES_JSON" >> "$GITHUB_OUTPUT"
    log_verbose "GitHub Actions output set: changed=$FILES_JSON"
  fi

  exit 0
}

main "$@"
