// cwes : ["cwe127", "cwe188"]

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// struct used to force the memory layout
struct frame {
  char secret[32];
  char buffer[32];
};

char *vuln(int index, int size) {
  struct frame f = {
      .secret = "guacamole",
      .buffer = "Lorem ipsum dolor sit amet, con",
  };

  printf("secret : %p\n", f.secret);
  printf("buffer : %p\n", f.buffer);
  printf("bad : %p\n", f.buffer + index);

  if (index + size < 32 && index < 32) {
    char *ret = malloc(size);
    memcpy(ret, f.buffer + index, size);
    return ret;
  }

  return NULL;
}

int main() {
  printf("%s\n", vuln(5, 10));   // fine
  printf("%s\n", vuln(-32, 10)); // vuln
}
