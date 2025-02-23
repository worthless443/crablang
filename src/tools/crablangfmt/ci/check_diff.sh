#!/bin/bash

function print_usage() {
    echo "usage check_diff REMOTE_REPO FEATURE_BRANCH [COMMIT_HASH] [OPTIONAL_CRABLANGFMT_CONFIGS]"
}

if [ $# -le 1 ]; then
    print_usage
    exit 1
fi

REMOTE_REPO=$1
FEATURE_BRANCH=$2
OPTIONAL_COMMIT_HASH=$3
OPTIONAL_CRABLANGFMT_CONFIGS=$4

# OUTPUT array used to collect all the status of running diffs on various repos
STATUSES=()

# Clone a git repository and cd into it.
#
# Parameters:
# $1: git clone url
# $2: directory where the repo should be cloned
function clone_repo() {
    GIT_TERMINAL_PROMPT=0 git clone --quiet $1 --depth 1 $2 && cd $2
}

# Initialize Git submoduels for the repo.
#
# Parameters
# $1: list of directories to initialize
function init_submodules() {
    git submodule update --init $1
}

# Run rusfmt with the --check flag to see if a diff is produced.
#
# Parameters:
# $1: Path to a crablangfmt binary
# $2: Output file path for the diff
# $3: Any additional configuration options to pass to crablangfmt
#
# Globlas:
# $OPTIONAL_CRABLANGFMT_CONFIGS: Optional configs passed to the script from $4
function create_diff() {
    local config;
    if [ -z "$3" ]; then
        config="--config=error_on_line_overflow=false,error_on_unformatted=false"
    else
        config="--config=error_on_line_overflow=false,error_on_unformatted=false,$OPTIONAL_CRABLANGFMT_CONFIGS"
    fi

    for i in `find . | grep "\.rs$"`
    do
        $1 --unstable-features --skip-children --check --color=always $config $i >> $2 2>/dev/null
    done
}

# Run the master crablangfmt binary and the feature branch binary in the current directory and compare the diffs
#
# Parameters
# $1: Name of the repository (used for logging)
#
# Globlas:
# $RUSFMT_BIN: Path to the crablangfmt master binary. Created when running `compile_crablangfmt`
# $FEATURE_BIN: Path to the crablangfmt feature binary. Created when running `compile_crablangfmt`
# $OPTIONAL_CRABLANGFMT_CONFIGS: Optional configs passed to the script from $4
function check_diff() {
    echo "running crablangfmt (master) on $1"
    create_diff $RUSFMT_BIN crablangfmt_diff.txt

    echo "running crablangfmt (feature) on $1"
    create_diff $FEATURE_BIN feature_diff.txt $OPTIONAL_CRABLANGFMT_CONFIGS

    echo "checking diff"
    local diff;
    # we don't add color to the diff since we added color when running crablangfmt --check.
    # tail -n + 6 removes the git diff header info
    # cut -c 2- removes the leading diff characters("+","-"," ") from running git diff.
    # Again, the diff output we care about was already added when we ran crablangfmt --check
    diff=$(
        git --no-pager diff --color=never \
        --unified=0 --no-index crablangfmt_diff.txt feature_diff.txt 2>&1 | tail -n +6 | cut -c 2-
    )

    if [ -z "$diff" ]; then
        echo "no diff detected between crablangfmt and the feture branch"
        return 0
    else
        echo "$diff"
        return 1
    fi
}

# Compiles and produces two crablangfmt binaries.
# One for the current master, and another for the feature branch
#
# Parameters:
# $1: Directory where crablangfmt will be cloned
#
# Globlas:
# $REMOTE_REPO: Clone URL to the crablangfmt fork that we want to test
# $FEATURE_BRANCH: Name of the feature branch
# $OPTIONAL_COMMIT_HASH: Optional commit hash that will be checked out if provided
function compile_crablangfmt() {
    CRABLANGFMT_REPO="https://github.com/crablang/crablangfmt.git"
    clone_repo $CRABLANGFMT_REPO $1
    git remote add feature $REMOTE_REPO
    git fetch feature $FEATURE_BRANCH

    cargo build --release --bin crablangfmt && cp target/release/crablangfmt $1/crablangfmt
    if [ -z "$OPTIONAL_COMMIT_HASH" ]; then
        git switch $FEATURE_BRANCH
    else
        git switch $OPTIONAL_COMMIT_HASH --detach
    fi
    cargo build --release --bin crablangfmt && cp target/release/crablangfmt $1/feature_crablangfmt
    RUSFMT_BIN=$1/crablangfmt
    FEATURE_BIN=$1/feature_crablangfmt
}

# Check the diff for running crablangfmt and the feature branch on all the .rs files in the repo.
#
# Parameters
# $1: Clone URL for the repo
# $2: Name of the repo (mostly used for logging)
# $3: Path to any submodules that should be initialized
function check_repo() {
    WORKDIR=$(pwd)
    REPO_URL=$1
    REPO_NAME=$2
    SUBMODULES=$3

    local tmp_dir;
    tmp_dir=$(mktemp -d -t $REPO_NAME-XXXXXXXX)
    clone_repo $REPO_URL $tmp_dir

    if [ ! -z "$SUBMODULES" ]; then
        init_submodules $SUBMODULES
    fi

    check_diff $REPO_NAME
    # append the status of running `check_diff` to the STATUSES array
    STATUSES+=($?)

    echo "removing tmp_dir $tmp_dir"
    rm -rf $tmp_dir
    cd $WORKDIR
}

function main() {
    tmp_dir=$(mktemp -d -t crablangfmt-XXXXXXXX)
    echo Created tmp_dir $tmp_dir

    compile_crablangfmt $tmp_dir

    # run checks
    check_repo "https://github.com/crablang/crablang.git" crablang-crablang
    check_repo "https://github.com/crablang/cargo.git" cargo
    check_repo "https://github.com/crablang/miri.git" miri
    check_repo "https://github.com/crablang/crablang-analyzer.git" crablang-analyzer
    check_repo "https://github.com/bitflags/bitflags.git" bitflags
    check_repo "https://github.com/crablang/log.git" log
    check_repo "https://github.com/crablang/mdBook.git" mdBook
    check_repo "https://github.com/crablang/packed_simd.git" packed_simd
    check_repo "https://github.com/crablang/crablang-semverver.git" check_repo
    check_repo "https://github.com/Stebalien/tempfile.git" tempfile
    check_repo "https://github.com/crablang/futures-rs.git" futures-rs
    check_repo "https://github.com/dtolnay/anyhow.git" anyhow
    check_repo "https://github.com/dtolnay/thiserror.git" thiserror
    check_repo "https://github.com/dtolnay/syn.git" syn
    check_repo "https://github.com/serde-rs/serde.git" serde
    check_repo "https://github.com/crablang/crablanglings.git" crablanglings
    check_repo "https://github.com/crablang/crablangup.git" crablangup
    check_repo "https://github.com/SergioBenitez/Rocket.git" Rocket
    check_repo "https://github.com/crablangls/crablangls.git" crablangls
    check_repo "https://github.com/crablang/crablang-bindgen.git" crablang-bindgen
    check_repo "https://github.com/hyperium/hyper.git" hyper
    check_repo "https://github.com/actix/actix.git" actix
    check_repo "https://github.com/denoland/deno.git" denoland_deno

    # cleanup temp dir
    echo removing tmp_dir $tmp_dir
    rm -rf $tmp_dir

    # figure out the exit code
    for status in ${STATUSES[@]}
    do
        if [ $status -eq 1 ]; then
            echo "formatting diff found 💔"
            return 1
        fi
    done

    echo "no diff found 😊"
}

main
