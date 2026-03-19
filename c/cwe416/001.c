// cwes : ["ccwe446"]
// desc : "Small cli application where a user has three actions that manipulate memory."

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SZ 32

void vuln() {
    volatile char *culprit = malloc(SZ);
    memset((void*)culprit, 0, SZ);

    printf("culprit alloc @ %p\n", (void*)culprit);

    free((void*)culprit);

    char *arr = malloc(SZ);
    memset(arr, 'A', SZ);

    printf("arr alloc @ %p\n", (void*)arr);

    if (culprit[4] == 'A') {
        puts("use-after-free observed");
    } else {
        puts("reuse did not occur (still UAF)");
    }
}

int main(void) {
    vuln();
}
