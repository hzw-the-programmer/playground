psCipherContext_t ctx = {0};

assert(out_len>=in_len+AES_BLOCK_SIZE-in_len%AES_BLOCK_SIZE);
assert(key_len == AES_256_KEY_LEN);
assert(iv_len == AES_IV_LEN);

psAesInit(&ctx, iv, key, key_len);

while (in_len > AES_BLOCK_SIZE)
{
    psAesEncrypt(&ctx, in, out, AES_BLOCK_SIZE);

    in += AES_BLOCK_SIZE;
    out += AES_BLOCK_SIZE;
    in_len -= AES_BLOCK_SIZE;
}

if (in_len)
{
    uint8_t tmp[AES_BLOCK_SIZE] = {0};
    uint8_t padding_len = 0;

    memcpy(tmp, in, in_len);
    padding_len = AES_BLOCK_SIZE - in_len;
    if (padding_len == 0)
    {
        psAesEncrypt(&ctx, tmp, out, AES_BLOCK_SIZE);
        out += AES_BLOCK_SIZE;
        memset(tmp, AES_BLOCK_SIZE, AES_BLOCK_SIZE);
        psAesEncrypt(&ctx, tmp, out, AES_BLOCK_SIZE);
    }
    else if (padding_len > 0)
    {
        memset(tmp + in_len, padding_len, padding_len);
        psAesEncrypt(&ctx, tmp, out, AES_BLOCK_SIZE);
    }
}

# 2
psCipherContext_t ctx = {0};
int32_t len = 0;
uint8_t tmp[AES_BLOCK_SIZE] = {0};
uint8_t padding_len = 0;

assert(out_len>=in_len+AES_BLOCK_SIZE-in_len%AES_BLOCK_SIZE);
assert(key_len == AES_256_KEY_LEN);
assert(iv_len == AES_IV_LEN);

psAesInit(&ctx, iv, key, key_len);

len = in_len - in_len % AES_BLOCK_SIZE;
if (len)
{
    psAesEncrypt(&ctx, in, out, len);
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

psAesEncrypt(&ctx, tmp, out, AES_BLOCK_SIZE);