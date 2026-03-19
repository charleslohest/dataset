// cwes : ["ccwe446"]
// desc : "Small cli application where a user has three actions that manipulate memory."

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SZ 32
#define BIGBUF_SZ 64

void vuln(void) {
  volatile char *culprit = NULL;
  char *array = NULL;
  char buffer[BIGBUF_SZ];

  while (1) {
    if (!fgets(buffer, BIGBUF_SZ, stdin))
      break;

    /* strip newline */
    buffer[strcspn(buffer, "\n")] =
        '\0'; // replace the index of \n with null term

    if (strcmp(buffer, "init culprit") == 0) {
      culprit = (volatile char *)malloc(SZ);
      memset((void *)culprit, 0, SZ);
      printf("culprit allocated @ %p\n", (void *)culprit);

    } else if (strcmp(buffer, "get culprit") == 0) {
      if (!culprit) {
        puts("culprit is NULL");
        continue;
      }
      for (int i = 0; i < SZ; ++i)
        printf("[%d] %c\n", i, culprit[i]);

    } else if (strcmp(buffer, "kill culprit") == 0) {
      free((void *)culprit);
      printf("culprit freed\n");

    } else if (strcmp(buffer, "init array") == 0) {
      array = malloc(SZ);
      memset(array, 'A', SZ);
      printf("array allocated @ %p\n", (void *)array);

    } else if (strcmp(buffer, "get array") == 0) {
      if (!array) {
        puts("array is NULL");
        continue;
      }
      for (int i = 0; i < SZ; ++i)
        printf("[%d] %c\n", i, array[i]);

    } else if (strcmp(buffer, "kill array") == 0) {
      free(array);
      printf("array freed\n");

    } else {
      puts("unknown command");
    }
  }
}

int main(void) { vuln(); }
