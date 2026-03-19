C_SOURCE := $(wildcard *.c)
TARGETS  := $(C_SOURCE:.c=)

CC      ?= gcc
CFLAGS += 

all: $(TARGETS)
%: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(TARGETS)

.PHONY: all clean
