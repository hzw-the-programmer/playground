#include <stdint.h>
#include <string.h>
#include <assert.h>
#include "sha256.h"
#include "../utils.h"

void test_sha256() {
    char *tests[] = {
        "",
        "\xe3\xb0\xc4\x42\x98\xfc\x1c\x14\x9a\xfb\xf4\xc8\x99\x6f\xb9\x24\x27\xae\x41\xe4\x64\x9b\x93\x4c\xa4\x95\x99\x1b\x78\x52\xb8\x55",

        "abc",
        "\xba\x78\x16\xbf\x8f\x01\xcf\xea\x41\x41\x40\xde\x5d\xae\x22\x23\xb0\x03\x61\xa3\x96\x17\x7a\x9c\xb4\x10\xff\x61\xf2\x00\x15\xad",

        "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "\x24\x8d\x6a\x61\xd2\x06\x38\xb8\xe5\xc0\x26\x93\x0c\x3e\x60\x39\xa3\x3c\xe4\x59\x64\xff\x21\x67\xf6\xec\xed\xd4\x19\xdb\x06\xc1",

        "The quick brown fox jumps over the lazy dog",
        "\xd7\xa8\xfb\xb3\x07\xd7\x80\x94\x69\xca\x9a\xbc\xb0\x08\x2e\x4f\x8d\x56\x51\xe4\x6d\x3c\xdb\x76\x2d\x02\xd0\xbf\x37\xc9\xe5\x92",

        "The quick brown fox jumps over the lazy cog",
        "\xe4\xc4\xd8\xf3\xbf\x76\xb6\x92\xde\x79\x1a\x17\x3e\x05\x32\x11\x50\xf7\xa3\x45\xb4\x64\x84\xfe\x42\x7f\x6a\xcc\x7e\xcc\x81\xbe",

        "bhn5bjmoniertqea40wro2upyflkydsibsk8ylkmgbvwi420t44cq034eou1szc1k0mk46oeb7ktzmlxqkbte2sy",
        "\x90\x85\xdf\x2f\x02\xe0\xcc\x45\x59\x28\xd0\xf5\x1b\x27\xb4\xbf\x1d\x9c\xd2\x60\xa6\x6e\xd1\xfd\xa1\x1b\x0a\x3f\xf5\x75\x6d\x99",
    };
    int i;

    blk_SHA256_CTX ctx;
    char res[32];

    for (i = 0; i < ARRAY_SIZE(tests); i+=2) {
        blk_SHA256_Init(&ctx);
        blk_SHA256_Update(&ctx, tests[i], strlen(tests[i]));
        blk_SHA256_Final(res, &ctx);
        assert(memcmp(res, tests[i+1], 32) == 0);
    }
}