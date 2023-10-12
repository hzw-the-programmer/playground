STCHE ctx = {0};

assert(out_len>=in_len+AES_BLOCK_SIZE-in_len%AES_BLOCK_SIZE);
assert(key_len == AES_256_KEY_LEN);
assert(iv_len == AES_IV_LEN);

che_init(&ctx, CHE_AES);

che_key_action(&ctx, CHE_SET_KEY, key, key_len);
che_set_iv(&ctx, iv, iv_len);

while (in_len > AES_BLOCK_SIZE)
{
    che_process(&ctx,
        CHE_AES,
        CHE_CBC,
        CHE_ENC,
        in,
        out,
        AES_BLOCK_SIZE,
        false);

    in += AES_BLOCK_SIZE;
    out += AES_BLOCK_SIZE;
    in_len -= AES_BLOCK_SIZE;
}

if (in_len)
{
    che_process(&ctx,
        CHE_AES,
        CHE_CBC,
        CHE_ENC,
        in,
        out,
        in_len,
        true);
}

che_deinit(&ctx);

# 2
STCHE ctx = {0};
int32_t len = 0;
uint8_t tmp[AES_BLOCK_SIZE] = {0};
uint8_t padding_len = 0;

assert(in_len >= 0);
assert(out_len>=in_len+AES_BLOCK_SIZE-in_len%AES_BLOCK_SIZE);
assert(key_len == AES_256_KEY_LEN);
assert(iv_len == AES_IV_LEN);

che_init(&ctx, CHE_AES);

che_key_action(&ctx, CHE_SET_KEY, (uint8_t*)key, key_len);
che_set_iv(&ctx, (uint8_t*)iv, iv_len);

len = in_len - in_len % AES_BLOCK_SIZE;
if (len)
{
    che_process(&ctx,
        CHE_AES,
        CHE_CBC,
        CHE_ENC,
        (uint8_t*)in,
        out,
        len,
        false);
    in += len;
    in_len -= len;
    out += len;
}

if (in_len)
{
    memcpy(tmp, in, in_len);
    padding_len = AES_BLOCK_SIZE - in_len;
    memset(tmp + in_len, padding_len, padding_len);
}
else
{
    padding_len = AES_BLOCK_SIZE;
    memset(tmp, padding_len, padding_len);
}

che_process(&ctx,
    CHE_AES,
    CHE_CBC,
    CHE_ENC,
    (uint8_t*)tmp,
    out,
    AES_BLOCK_SIZE,
    false);

che_deinit(&ctx);