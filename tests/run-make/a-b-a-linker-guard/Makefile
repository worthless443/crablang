# ignore-cross-compile
include ../tools.mk

# Test that if we build `b` against a version of `a` that has one set
# of types, it will not run with a dylib that has a different set of
# types.

# NOTE(eddyb) this test only works with the `legacy` mangling,
# and will probably get removed once `legacy` is gone.

all:
	$(CRABLANGC) a.rs --cfg x -C prefer-dynamic -Z unstable-options -C symbol-mangling-version=legacy
	$(CRABLANGC) b.rs -C prefer-dynamic -Z unstable-options -C symbol-mangling-version=legacy
	$(call RUN,b)
	$(CRABLANGC) a.rs --cfg y -C prefer-dynamic -Z unstable-options -C symbol-mangling-version=legacy
	$(call FAIL,b)
