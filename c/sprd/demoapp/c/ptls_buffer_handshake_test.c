#include "compat.h"

#if defined(PICOTLS_SUPPORT)
#include "picotls.h"
#include "picotls/minicrypto.h"

uint8_t g_certificate[] = {
	0x30, 0x82, 0x02, 0x60, 0x30, 0x82, 0x01, 0x48, 0xa0, 0x03,
	0x02, 0x01, 0x02, 0x02, 0x01, 0x01, 0x30, 0x0d, 0x06, 0x09,
	0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b, 0x05,
	0x00, 0x30, 0x1a, 0x31, 0x18, 0x30, 0x16, 0x06, 0x03, 0x55,
	0x04, 0x03, 0x13, 0x0f, 0x70, 0x69, 0x63, 0x6f, 0x74, 0x6c,
	0x73, 0x20, 0x74, 0x65, 0x73, 0x74, 0x20, 0x63, 0x61, 0x30,
	0x1e, 0x17, 0x0d, 0x31, 0x38, 0x30, 0x32, 0x32, 0x33, 0x30,
	0x35, 0x33, 0x31, 0x30, 0x34, 0x5a, 0x17, 0x0d, 0x32, 0x38,
	0x30, 0x32, 0x32, 0x31, 0x30, 0x35, 0x33, 0x31, 0x30, 0x34,
	0x5a, 0x30, 0x1b, 0x31, 0x19, 0x30, 0x17, 0x06, 0x03, 0x55,

	0x04, 0x03, 0x13, 0x10, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x65,
	0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d,
	0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce,
	0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
	0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0xda, 0xc8, 0xa5,
	0x40, 0x54, 0xba, 0x33, 0xda, 0x18, 0xa9, 0x41, 0x7f, 0x49,
	0x53, 0xdf, 0x60, 0xe6, 0xa6, 0x3d, 0xb6, 0x8e, 0x53, 0x3a,
	0x9f, 0xdd, 0x19, 0x14, 0x5e, 0xab, 0x03, 0xcf, 0xbc, 0xfb,
	0x36, 0x98, 0x16, 0x24, 0x8f, 0x07, 0x29, 0x6d, 0x15, 0xd8,
	0x4f, 0x30, 0xe8, 0x09, 0x64, 0xfb, 0x14, 0xfc, 0x86, 0x7c,

	0xd4, 0x06, 0xc2, 0xfd, 0x9d, 0xe8, 0x99, 0x3f, 0x48, 0x8c,
	0x2b, 0xa3, 0x7b, 0x30, 0x79, 0x30, 0x09, 0x06, 0x03, 0x55,
	0x1d, 0x13, 0x04, 0x02, 0x30, 0x00, 0x30, 0x2c, 0x06, 0x09,
	0x60, 0x86, 0x48, 0x01, 0x86, 0xf8, 0x42, 0x01, 0x0d, 0x04,
	0x1f, 0x16, 0x1d, 0x4f, 0x70, 0x65, 0x6e, 0x53, 0x53, 0x4c,
	0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64,
	0x20, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
	0x74, 0x65, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x1d, 0x0e, 0x04,
	0x16, 0x04, 0x14, 0xee, 0x30, 0x86, 0x16, 0xa1, 0xd2, 0x69,
	0xad, 0x64, 0xe4, 0xd7, 0x77, 0x6b, 0xb2, 0xfd, 0x5c, 0x4f,

	0x01, 0xa2, 0xb5, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x1d, 0x23,
	0x04, 0x18, 0x30, 0x16, 0x80, 0x14, 0xbf, 0x79, 0xca, 0x97,
	0xb2, 0x60, 0x78, 0x20, 0x96, 0xaa, 0x46, 0x57, 0x9c, 0xdf,
	0xa7, 0xb2, 0x23, 0xf5, 0x25, 0x63, 0x30, 0x0d, 0x06, 0x09,
	0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b, 0x05,
	0x00, 0x03, 0x82, 0x01, 0x01, 0x00, 0x8f, 0xac, 0x9c, 0x01,
	0x6d, 0x81, 0xaa, 0x8c, 0xae, 0x5d, 0xb5, 0x16, 0x74, 0xea,
	0xe8, 0xeb, 0x26, 0x5b, 0xb1, 0x66, 0xd5, 0x6b, 0xd4, 0x4d,
	0x79, 0x0d, 0x6d, 0x87, 0xa9, 0xb6, 0xbf, 0x74, 0x2d, 0xc1,
	0xb2, 0x2e, 0x52, 0xb6, 0x4b, 0xca, 0x0d, 0x01, 0x45, 0x38,

	0x58, 0x1a, 0xd2, 0x6a, 0x6d, 0x20, 0x98, 0x5a, 0x51, 0xb0,
	0x6f, 0x2c, 0x3f, 0x0f, 0x12, 0x88, 0xed, 0x7c, 0x09, 0xa5,
	0x74, 0x00, 0x21, 0x3d, 0x4b, 0xd2, 0x2d, 0x54, 0xaa, 0x53,
	0x8b, 0x64, 0xf9, 0x1e, 0xea, 0xa5, 0x8a, 0xe7, 0x61, 0x5e,
	0x56, 0x92, 0x52, 0x36, 0x3e, 0xa0, 0x68, 0x59, 0x9c, 0x7d,
	0xb3, 0xe8, 0x5c, 0x4b, 0x77, 0x6e, 0xde, 0x28, 0xed, 0x18,
	0x91, 0xa9, 0x9c, 0x39, 0xd2, 0x96, 0xcc, 0x98, 0x05, 0x8c,
	0x74, 0xdc, 0x1e, 0x12, 0x5b, 0x38, 0xbd, 0x56, 0xcb, 0xa3,
	0xe8, 0xe1, 0x2a, 0x5a, 0x2b, 0xd2, 0x32, 0x45, 0xc1, 0x10,
	0x85, 0x20, 0x6c, 0x6b, 0x34, 0xea, 0x66, 0x91, 0x0e, 0x2e,

	0xb8, 0x64, 0x87, 0x9f, 0x07, 0xbc, 0x23, 0x4f, 0x23, 0xad,
	0xbe, 0x89, 0xdf, 0x0a, 0x98, 0x47, 0xe9, 0x63, 0x02, 0xd3,
	0x41, 0xf4, 0x2d, 0xa4, 0xce, 0xdd, 0xe3, 0xd8, 0x41, 0x08,
	0xfe, 0xdf, 0x47, 0xc0, 0xe7, 0x63, 0x8e, 0x1f, 0xf0, 0x4b,
	0xc5, 0xae, 0xab, 0xc0, 0xba, 0x38, 0x3e, 0xe3, 0x90, 0x9c,
	0x08, 0xbd, 0x75, 0x1c, 0xb9, 0xb8, 0x54, 0x43, 0x1d, 0x99,
	0x42, 0xe0, 0xa2, 0xb7, 0x75, 0xbb, 0x14, 0x03, 0x79, 0x9a,
	0xf6, 0x07, 0xd8, 0xa5, 0xab, 0x2b, 0x3a, 0x70, 0x8b, 0x77,
	0x85, 0x70, 0x8a, 0x98, 0x38, 0x9b, 0x35, 0x09, 0xf6, 0x62,
	0x6b, 0x29, 0x4a, 0xa7, 0xa7, 0xf9, 0x3b, 0xde, 0xd8, 0xc8,

	0x90, 0x57, 0xf2, 0x76, 0x2a, 0x23, 0x0b, 0x01, 0x68, 0xc6,
	0x9a, 0xf2,	
};

uint8_t ecdsa_key[] = {
	0xc1, 0x74, 0xb4, 0xf9, 0x5e, 0xfe, 0x7a, 0x01, 0x0e, 0xbe,
	0x4a, 0xe8, 0x33, 0xb2, 0x36, 0x13, 0xfc, 0x65, 0xe9, 0x65,
	0x91, 0xa8, 0x39, 0x9e, 0x9a, 0x80, 0xfb, 0xab, 0xd1, 0xff,
	0xba, 0x3a,
};

#define REQUEST  "GET / HTTP/1.1\r\n\r\n"
#define RESPONSE "HTTP/1.1 200 OK\r\n\r\n"

static void picotls_test_1()
{
	char *str = "hello picotls";
	ptls_buffer_t buf;
	int ret;

    LOG("***picotls_test_1:begin***");

	ptls_buffer_init(&buf, "", 0);
	ptls_buffer_pushv(&buf, str, strlen(str));

    LOG_HEXDUMP(buf.base, buf.off);

Exit:
	ptls_buffer_dispose(&buf);    
    LOG("***picotls_test_1:end***");
}

static void picotls_test_2()
{
	ptls_context_t ctx = {0};
	ptls_t *tls;
	ptls_buffer_t buf;
    int ret;

	ctx.get_time = &ptls_get_time;
	ctx.random_bytes = ptls_minicrypto_random_bytes;
	ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	ctx.cipher_suites = ptls_minicrypto_cipher_suites;

	tls = ptls_client_new(&ctx);

	ptls_buffer_init(&buf, "", 0);
	ret = ptls_handshake(tls, &buf, NULL, 0, NULL);
    LOG("picotls_test_2: 0x%x", ret);

	ptls_buffer_dispose(&buf);
	ptls_free(tls);
}

static void picotls_test_3()
{
	ptls_context_t ctx = { 0 };
	ptls_t* tls;
	ptls_buffer_t buf;

	ptls_context_t server_ctx = { 0 };
	ptls_t* server_tls;
	ptls_buffer_t server_buf;
	size_t len;
	int ret;
	ptls_iovec_t certificate = { g_certificate, sizeof(g_certificate)};
	ptls_minicrypto_secp256r1sha256_sign_certificate_t minicrypto_sign_certificate = {0};

	ctx.get_time = &ptls_get_time;
	ctx.random_bytes = ptls_minicrypto_random_bytes;
	ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	ctx.cipher_suites = ptls_minicrypto_cipher_suites;

    slice_t s;

	tls = ptls_client_new(&ctx);

	server_ctx.get_time = &ptls_get_time;
	server_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	server_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	server_ctx.random_bytes = ptls_minicrypto_random_bytes;

	server_ctx.certificates.list = &certificate;
	server_ctx.certificates.count = 1;

	ptls_minicrypto_init_secp256r1sha256_sign_certificate(&minicrypto_sign_certificate, ptls_iovec_init(ecdsa_key, sizeof(ecdsa_key)));
	server_ctx.sign_certificate = &minicrypto_sign_certificate.super;

	server_tls = ptls_server_new(&server_ctx);

	ptls_buffer_init(&buf, "", 0);
	ptls_buffer_init(&server_buf, "", 0);

	ret = ptls_handshake(tls, &buf, NULL, 0, NULL);
    LOG("client:ptls_handshake: ret=0x%x, len=0x%x", ret, buf.off);

	len = buf.off;
	buf.off = 0;
	ret = ptls_handshake(server_tls, &server_buf, buf.base, &len, NULL);
    LOG("server:ptls_handshake: ret=0x%x, len=0x%x", ret, server_buf.off);

    len = server_buf.off;
    server_buf.off = 0;
	ret = ptls_handshake(tls, &buf, server_buf.base, &len, NULL);
    LOG("client:ptls_handshake: ret=0x%x, len=0x%x", ret, buf.off);

    ret = ptls_send(tls, &buf, REQUEST, strlen(REQUEST));
    LOG("client:ptls_send: ret=0x%x, len=0x%x", ret, buf.off);

	len = buf.off;
    buf.off = 0;
	ret = ptls_receive(server_tls, &server_buf, buf.base, &len);
    LOG("server:ptls_receive: ret=0x%x, len=0x%x", ret, server_buf.off);
    LOG("server:ptls_receive");
    s = slice_new(server_buf.base, server_buf.off);
    LOG("%S", &s);

    server_buf.off = 0;
	ret = ptls_send(server_tls, &server_buf, RESPONSE, strlen(RESPONSE));
    LOG("server:ptls_send: ret=0x%x, len=0x%x", ret, server_buf.off);
	
	len = server_buf.off;
    server_buf.off = 0;
	ret = ptls_receive(tls, &buf, server_buf.base, &len);
    LOG("client:ptls_receive: ret=0x%x, len=0x%x", ret, buf.off);
    LOG("client:ptls_receive");
    s = slice_new(buf.base, buf.off);
    LOG("%S", &s);

	ptls_buffer_dispose(&buf);
	ptls_free(tls);

	ptls_buffer_dispose(&server_buf);
	ptls_free(server_tls);
}

void picotls_test()
{
	picotls_test_1();
	picotls_test_2();
	picotls_test_3();
}
#endif // PICOTLS_SUPPORT
