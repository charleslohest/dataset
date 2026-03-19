// cwes : ["cwe188"]
// desc : "Not a proper vulnerability but dangerous behaviour"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

struct MetaData {
  char name[32];
  char version[8];
};

union packet {
  struct MetaData metadata;
  uint8_t buffer[sizeof(struct MetaData)];
};

void vuln() {
  union packet p = {
      .metadata =
          {
              .name = "I love this code",
              .version = "1",
          },
  };

  unsigned int start = strcspn(p.metadata.name, "l");
  unsigned int overwrite = 32 - start;

  unsigned int padding_size = overwrite - 13;
  char padding[padding_size];
  memset((void *)padding, ' ', padding_size);
  padding[padding_size - 1] = '\0';

  printf("%s\n", p.metadata.name);
  printf("%s\n", p.metadata.version);

  snprintf((void *)p.buffer + start, overwrite + 8, "%s%s%s",
           "hate this code", padding, "2");
  p.buffer[31] = '\0';

  printf("%s\n", p.metadata.name);
  printf("%s\n", p.metadata.version);
}

int main() {
  vuln();
  return 0;
}
