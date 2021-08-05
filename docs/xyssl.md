```
#include "xyssl_interface.h"

void* sslmalloc(int mem_size, void *user_data) {
    return malloc(mem_size);
}

void sslfree(void *mem_ptr, void *user_data) {
    free(mem_ptr);
}

static int rand_func(void *p) {
    return rand()<<16 | rand();
}

static void rsa_test() {
    ssl_context sslCtx;
    rsa_context rsaCtx;
    int ret, pt1l = 13;
    char pt[13] = {0};
    unsigned char ct[128] = {0};
    char pt1[13] = {0};
    RA_UINT32 start;

    xyssl_adp_ssl_init(&sslCtx, sslmalloc, sslfree, NULL);
    xyssl_rsa_init(&rsaCtx, RSA_PKCS_V15, RSA_SHA256, rand_func, NULL);

    xyssl_adp_RAND_seed();
    start = kal_get_systicks();
    ret = xyssl_rsa_gen_key(&sslCtx, &rsaCtx, 512, 37);
    kal_prompt_trace(MOD_ABM, "xyssl_rsa_gen_key cost=%d", kal_get_systicks() - start);

    strcpy(pt, "hello world!");    

    start = kal_get_systicks();
    ret = xyssl_rsa_pkcs1_encrypt(&sslCtx, &rsaCtx, RSA_PUBLIC, strlen(pt), pt, ct);    
    kal_prompt_trace(MOD_ABM, "xyssl_rsa_pkcs1_encrypt cost=%d", kal_get_systicks() - start);

    start = kal_get_systicks();
    ret = xyssl_rsa_pkcs1_decrypt(&sslCtx, &rsaCtx, RSA_PRIVATE, &pt1l, ct, pt1);
    kal_prompt_trace(MOD_ABM, "xyssl_rsa_pkcs1_decrypt cost=%d", kal_get_systicks() - start);

    rsa_free(&sslCtx, &rsaCtx);
    xyssl_adp_ssl_deinit(&sslCtx);

    kal_prompt_trace(MOD_ABM, "pt1=%s", pt1);
}
```
