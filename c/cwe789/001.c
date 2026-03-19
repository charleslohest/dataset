#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int proc_msg(const char *s, int msg_len) {
  int pre_len = sizeof("preamble: ") - 1; // exclude null terminator
  int buf_len = pre_len - msg_len;

  printf("[proc_msg] pre_len=%d, msg_len=%d, buf_len=%d\n", pre_len, msg_len,
         buf_len);

  // CWE-789 : uncontrolled allocation size
  char *buf = malloc(buf_len);

  if (!buf) {
    printf("[proc_msg] malloc failed (likely huge allocation)\n");
    return -1;
  }

  /* Dummy processing */
  strncpy(buf, s, buf_len);
  buf[buf_len - 1] = '\0';

  printf("[proc_msg] buf=\"%s\"\n", buf);
  free(buf);
  return 0;
}

void vuln() {
  const char *s = "preamble: message\n";
  const char *sl = strchr(s, ':');

  int msg_len = (sl == NULL) ? 0 : (int)(sl - s);
  printf("[safe_call] msg_len=%d\n", msg_len);

  proc_msg(s, msg_len);

  const char *vuln = "preamble: message\n";
  msg_len = 100;

  printf("[vuln_call] msg_len=%d\n", msg_len);
  proc_msg(vuln, msg_len);
}

int main(void) { return 0; }
