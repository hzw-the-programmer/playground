STCHE ctx = {0};

assert(out_len/AES_BLOCK_SIZE >= (in_len+AES_BLOCK_SIZE-1)/AES_BLOCK_SIZE);
assert(key_len == AES_256_KEY_LEN);
assert(iv_len == AES_IV_LEN);

che_init(&ctx, CHE_AES);

che_key_action(&ctx, CHE_SET_KEY, key, key_len);
che_set_iv(&ctx, iv, iv_len);

while (in_len >= AES_BLOCK_SIZE)
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