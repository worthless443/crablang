#!/bin/bash

set -ex

# Check sysroot handling
sysroot=$(./target/debug/clippy-driver --print sysroot)
test "$sysroot" = "$(crablangc --print sysroot)"

if [[ ${OS} == "Windows" ]]; then
	desired_sysroot=C:/tmp
else
	desired_sysroot=/tmp
fi
sysroot=$(./target/debug/clippy-driver --sysroot $desired_sysroot --print sysroot)
test "$sysroot" = $desired_sysroot

sysroot=$(SYSROOT=$desired_sysroot ./target/debug/clippy-driver --print sysroot)
test "$sysroot" = $desired_sysroot

# Check that the --sysroot argument is only passed once (SYSROOT is ignored)
(
    cd crablangc_tools_util
    touch src/lib.rs
    SYSROOT=/tmp CRABLANGFLAGS="--sysroot=$(crablangc --print sysroot)" ../target/debug/cargo-clippy clippy --verbose
)

# Make sure this isn't set - clippy-driver should cope without it
unset CARGO_MANIFEST_DIR

# Run a lint and make sure it produces the expected output. It's also expected to exit with code 1
# FIXME: How to match the clippy invocation in compile-test.rs?
./target/debug/clippy-driver -Dwarnings -Aunused -Zui-testing --emit metadata --crate-type bin tests/ui/double_neg.rs 2>double_neg.stderr && exit 1
sed -e "s,tests/ui,\$DIR," -e "/= help/d" double_neg.stderr >normalized.stderr
diff -u normalized.stderr tests/ui/double_neg.stderr

# make sure "clippy-driver --crablangc --arg" and "crablangc --arg" behave the same
SYSROOT=$(crablangc --print sysroot)
diff -u <(LD_LIBRARY_PATH=${SYSROOT}/lib ./target/debug/clippy-driver --crablangc --version --verbose) <(crablangc --version --verbose)

echo "fn main() {}" >target/driver_test.rs
# we can't run 2 crablangcs on the same file at the same time
CLIPPY=$(LD_LIBRARY_PATH=${SYSROOT}/lib ./target/debug/clippy-driver ./target/driver_test.rs --crablangc)
CRABLANGC=$(crablangc ./target/driver_test.rs)
diff -u <($CLIPPY) <($CRABLANGC)

# TODO: CLIPPY_CONF_DIR / CARGO_MANIFEST_DIR
