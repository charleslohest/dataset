##
# FORCES dataset
#

SUBDIRS  = c rust
CC       = gcc
CFLAGS   = 
LDFLAGS  = 

RUSTCC   = rustc
RUSTFLAGS   = 

export CC CFLAGS LDFLAGS RUSTC RUSTFLAGS

release: CFLAGS += -O2
release: all

debug: CFLAGS += -O0 -g  \
  -fno-stack-protector   \
  -U_FORTIFY_SOURCE      \
  -D_FORTIFY_SOURCE=0    \
  -no-pie 
debug: all

all: 
	for dir in $(SUBDIRS); do     \
		$(MAKE) -C $$dir;         \
	done

clean: 
	for dir in $(SUBDIRS); do    \
		$(MAKE) -C $$dir clean;  \
	done


# end
