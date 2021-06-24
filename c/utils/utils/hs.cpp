#if 0
typedef char *hs;
typedef int hslt

#define hshl (sizeof(hslt) << 1)
#define hsallocp(s) (hslt*)(s - hshl)
#define hslenp(s) (hslt*)(s - (hshl >> 1))

hs hsnewlen(const void *init, size_t initlen) {
    void *sh;
    hs s;

    sh = malloc(hshl + initlen + 1);
    if (!sh) return NULL;
    s = sh + hshl;
    hsallocp(s) = initlen;
    hslenp(s) = initlen;

    if (init) {
        memcpy(s, init, initlen);
    }
    s[initlen] = '\0';

    return s;
}

void test_hsnewlen() {
    char *s1 = "hello world!";
    hs s;

    s = hsnewlen(s1, strlen(s1));
    assert(strlen(s) == *hsallocp(s));
}

void test() {
    tcp
}

void test_hs() {
    test_hsnewlen();
}
#else
void test_hs() {
}
#endif
