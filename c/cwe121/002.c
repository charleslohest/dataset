// https://tgrez.github.io/posts/2022-06-19-buffer-overflow-in-rust.html
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void abracadabra() {
 printf("Abracadabra! Function called!\n");
 exit(0);
}

int main(int argc, char **argv) {
  struct {
    char buffer[64];
    volatile int (*point)();
  } hackvist;

  hackvist.point = NULL;
  strcpy(hackvist.buffer, argv[1]);

  printf("abracadabra function address: %p\n",abracadabra);
  printf("hackvist.point after strcpy: %p\n", hackvist.point);
  if (hackvist.point) {
    fflush(stdout);
    hackvist.point();
  } else {
    printf("Try Again\n");
  }

  exit(0);
}


