#define _GNU_SOURCE

// cwes : [ "cwe119", "cwe121", "cwe787" ]
// desc : "A vulnerable dns resolver"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <errno.h>

#include <netdb.h>
#include <netinet/in.h>

#include <pthread.h>

/// returns the address or NULL
uint32_t *char_to_addr(char *user_supplied_addr) {

  char buffer[100] = {0}; // magic number

  // buffer overflow
  memcpy(buffer, user_supplied_addr,
         strlen(user_supplied_addr) // copy depends on size of user input
  );

  char *tok = strtok(buffer, ".");
  int counter = 0;
  uint8_t *acc = malloc(sizeof(uint8_t) * 4);

  // out of bounds write 
  while (tok) {
    acc[3 - counter] = atoi(tok); // endianess
    tok = strtok(NULL, ".");
    ++counter; // if the user provided address has >4 dots it does out of bound write
  }

  // we only check if the address contains 4 dots : a user can provide
  // infinitely many characters between those dots
  if (counter == 4) {
    return (uint32_t *)acc;
  } else {
    return NULL;
  }
}

void *resolve(void *addr) {
  return (void *)gethostbyaddr((in_addr_t *)addr, sizeof(struct in_addr),
                               AF_INET);
}

int vuln(char *user_supplied_addr) {
  struct hostent *hp;
  in_addr_t *addr;
  char hostname[64]; // buffer overflow
  in_addr_t inet_addr(const char *cp);

  /*routine that ensures user_supplied_addr is in the right format for
   * conversion */

  if ((addr = char_to_addr(user_supplied_addr)) != NULL) {
    struct timespec ts;
    pthread_t p;
    int s;

    pthread_create(&p, NULL, resolve, (void *)addr);
    if (clock_gettime(CLOCK_REALTIME, &ts) == -1) {
      return 1;
    }

    ts.tv_sec += 3; // wait for 3 seconds
    if (pthread_timedjoin_np(p, (void **)&hp, &ts) == 0) {
      // not checking if hp is null
      strcpy(hostname, hp->h_name);
      printf("the host is : %s\n", hp->h_name);
      return 0;
    } else {
      printf("[timeout] could not resolve the host for address : %s\n", user_supplied_addr);
    }

    return 1;
  } else {
    errno = EINVAL; // Set errno to an appropriate error code
    return 1;
  };
}

int main() {

  // not checking the ret val
  vuln("8.8.8.8");                    // intented use */
  vuln("192.168.1.1");                    // intented use */
  // will crash from here
  vuln("81.138.71.238.81.138.71.238.138.71.238.81.138.71.238.138.71.238.81.138.71.238.138.71.238.81.138.71.238"); // vuln */
  vuln("65465464.465464.644654.6456465"); // vuln */
  vuln("81.138.71.238"); // vuln */
  return 0;
}
