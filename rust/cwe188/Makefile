RUST_SOURCE := $(wildcard *.rs)
TARGETS  := $(RUST_SOURCE:.rs=)

RUSTCC ?= rustc

all: $(TARGETS)
%: %.rs
	$(RUSTCC) $(RUSTFLAGS) -o $@ $<

clean:
	rm -f $(TARGETS)

.PHONY: all clean
