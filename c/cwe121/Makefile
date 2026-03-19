C_SOURCE := $(wildcard *.c)
TARGETS  := $(C_SOURCE:.c=)

CC      ?= gcc

all: $(TARGETS)
%: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(TARGETS)

.PHONY: all clean
