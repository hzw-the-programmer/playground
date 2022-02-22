git://busybox.net/busybox.git
c31b54fd81690b3df3898437f5865674d06e6577

https://github.com/matrixssl/matrixssl.git

https://www.cnblogs.com/starwolf/p/3365834.html
https://crypto.stackexchange.com/questions/48628/why-is-padding-used-in-cbc-mode
https://stackoverflow.com/questions/8804574/aes-encryption-how-to-transport-iv

```
void psAesTest() {
    #define LEN 1024
    char iv[16] = {0};
    char key[32] = {0};    
    psCipherContext_t ctx;
    unsigned char src[LEN] = {0};
    unsigned char ct[LEN] = {0};
    unsigned char pt[LEN] = {0};
    unsigned char *srcp, *ctp, *ptp;
    int i = 0, j = 0;

    memset(iv, 0x01, sizeof(iv));
    memset(key, 0x02, sizeof(key));

    for (i = 0; i < 100; i++) {
        srcp = src;
        ctp = ct;
        ptp = pt;

        memset(src, i, LEN);
        memset(ct, 0, LEN);
        memset(pt, 0, LEN);

    #if 1
        memcpy(ct, src, LEN);

        psAesInit(&ctx, iv, key, 32);
        while (ctp < ct + LEN) {
            psAesEncrypt(&ctx, ctp, ctp, 16);
            ctp += 16;
        }

        ctp = ct;

        psAesInit(&ctx, iv, key, 32);
        while (ctp < ct + LEN) {
            psAesDecrypt(&ctx, ctp, ctp, 16);
            ctp += 16;
        }

        SCI_ASSERT(memcmp(ct, src, LEN) == 0);
    #else
        psAesInit(&ctx, iv, key, 32);
        while (srcp < src + LEN) {
            psAesEncrypt(&ctx, srcp, ctp, 16);
            srcp += 16;
            ctp += 16;
        }  

        ctp = ct;

        psAesInit(&ctx, iv, key, 32);
        while (ctp < ct + LEN) {
            psAesDecrypt(&ctx, ctp, ptp, 16);
            ctp += 16;
            ptp += 16;
        }

        SCI_ASSERT(memcmp(pt, src, LEN) == 0);
    #endif
    }
}
```
