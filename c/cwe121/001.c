// cwes : ["cwe121"]
// desc : "simple stack based buffer overlow, hard corded credentials"

#include <string.h>
#include <stdio.h>

#define BUFFER_SIZE 10

void vuln(const char *pwd) {
  char buf_pwd[BUFFER_SIZE] = {0};
  int is_password_good = 0;

  strcpy(buf_pwd, pwd);

  // hard coded credential
  if (strcmp(buf_pwd, "7c076e55-a9f1-4689-84c7-be825e3e1be4") == 0)
    is_password_good = 1;

  if (is_password_good > 0)
    printf("you found my cookie jar!\n");
  else
    printf("my cookies are safe :)\n");
}

int main(int argc, char *argv[]) {
  vuln(argv[1]);
  return 0;
}
