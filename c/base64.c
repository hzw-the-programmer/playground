#include <stdio.h>
#include <string.h>

static const unsigned char base64_enc_map[64] =
{
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/'
};

static const unsigned char base64_dec_map[128] =
{
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127,  62, 127, 127, 127,  63,  52,  53,
     54,  55,  56,  57,  58,  59,  60,  61, 127, 127,
    127,  64, 127, 127, 127,   0,   1,   2,   3,   4,
      5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
     25, 127, 127, 127, 127, 127, 127,  26,  27,  28,
     29,  30,  31,  32,  33,  34,  35,  36,  37,  38,
     39,  40,  41,  42,  43,  44,  45,  46,  47,  48,
     49,  50,  51, 127, 127, 127, 127, 127
};

int base64_decode(unsigned char *dst, size_t dlen, size_t *olen, const unsigned char *src, size_t slen) {
    int i, x, n, j;
    unsigned char *p;
    for (i = x = n = 0, j = 3, p = dst; i < slen; i++) {
        //printf("%c", src[i]);
        j -= base64_dec_map[src[i]] == 64;
        x = (x << 6) | (base64_dec_map[src[i]] & 0x3f);
        if (++n == 4) {
            n = 0;
            //printf(".%d", j);
            if (j > 0) {
                *p++ = (unsigned char)(x >> 16);
                //printf("%02x", *(p-1));
            }
            if (j > 1) {
                *p++ = (unsigned char)(x >> 8);
                //printf("%02x", *(p-1));
            }
            if (j > 2) {
                *p++ = (unsigned char)(x);
                //printf("%02x", *(p-1));
            }
            //printf(".");
        }
    }
    *olen = p - dst;
}

int base64_encode(unsigned char *dst, size_t dlen, size_t *olen, const unsigned char *src, size_t slen) {
    int i, c1, c2, c3, n;
    unsigned char *p;
    n = slen / 3 * 3;
    for (i = 0, p = dst; i < n; i+=3) {
        c1 = *src++;
        c2 = *src++;
        c3 = *src++;
        *p++ = base64_enc_map[c1 >> 2];
        *p++ = base64_enc_map[(c1 << 4 | c2 >> 4) & 0x3f];
        *p++ = base64_enc_map[(c2 << 2 | c3 >> 6) & 0x3f];
        *p++ = base64_enc_map[c3 & 0x3f];
    }
    if (i < slen) {
        c1 = *src++;
        c2 = (i + 1) < slen ? *src++ : 0;
        *p++ = base64_enc_map[c1 >> 2];
        *p++ = base64_enc_map[(c1 << 4 | c2 >> 4) & 0x3f];
        if (i + 1 < slen) *p++ = base64_enc_map[(c2 << 2) & 0x3f];
        else *p++ = '=';
        *p++ = '=';
    }
    *olen = p - dst;
    *p = 0;
}

void print_hex(unsigned char *c, size_t l) {
    for (int i = 0; i < l; i++) {
        printf("%02x", c[i]);
    }
}

struct {
    char *src;
    char *dst;
    size_t dlen;
} tests[] = {
    {
        "AQAEABgAAAEAAClyAf8DAKmNc10IAKu4",
        "\x01\x00\x04\x00\x18\x00\x00\x01\x00\x00\x29\x72\x01\xff\x03\x00\xa9\x8d\x73\x5d\x08\x00\xab\xb8",
        24
    },
    {
        "AgAHADcAAAEAAP6/Af8CAAAAAAAAAAAAAf9aAgECFiAAAAAAAAAAAAAAAAAAAAAAAAAAAADdiA==",
        "\x02\x00\x07\x00\x37\x00\x00\x01\x00\x00\xfe\xbf\x01\xff\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\xff\x5a\x02\x01\x02\x16\x20\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xdd\x88",
        55
    }
};

int main() {
    char dst[1024];
    size_t dlen = sizeof(dst);
    size_t olen;
    for (int i = 0; i < sizeof(tests) / sizeof(*tests); i++) {
        base64_decode(dst, dlen, &olen, tests[i].src, strlen(tests[i].src));
        if (tests[i].dlen != olen || memcmp(tests[i].dst, dst, olen)) {
            printf("%s\n", "decode error:");
            printf("expect: "); print_hex(tests[i].dst, tests[i].dlen); printf("\n");
            printf("actual: "); print_hex(dst, olen); printf("\n");
        }

        base64_encode(dst, dlen, &olen, tests[i].dst, tests[i].dlen);
        if (strlen(tests[i].src) != olen || memcmp(tests[i].src, dst, olen)) {
            printf("%s\n", "encode error:");
            printf("expect: %s\n", tests[i].src);
            printf("actual: %s\n", dst);
        }
    }
}
