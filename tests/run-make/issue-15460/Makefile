# ignore-cross-compile
include ../tools.mk

all: $(call NATIVE_STATICLIB,foo)
	$(CRABLANGC) foo.rs -C extra-filename=-383hf8 -C prefer-dynamic
	$(CRABLANGC) bar.rs
	$(call RUN,bar)
