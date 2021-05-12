#include <stdint.h>
#include <sys/types.h>

extern void printerr(char *file, int line, int err);
extern void die(char *file, int line, int err);

extern int hex_to_num(int hex);
extern int num_to_hex(int num, int lower);

extern size_t unhexlify(const char *str, size_t slen, uint8_t *data, size_t dlen);
extern size_t hexlify(const uint8_t *data, size_t dlen, char *str, size_t slen, int lower);
