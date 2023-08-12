#include "picotls.h"
#include "picotls/minicrypto.h"

#define EXTENSION_1 0x1234
#define EXTENSION_2 0x5678

typedef struct {
	size_t extension_data_len;
	uint8_t extension_data[32];
	ptls_raw_extension_t additional_extensions[3];
	ptls_handshake_properties_t properties;
} ctx_t;

uint8_t extension_1_data[] = "ext_1_data";
uint8_t extension_2_client_data[] = "ext_2_c_data";
uint8_t extension_2_server_data[] = "ext_2_server_data";

char test_sni[] = "test.example.com";
char test_alpn[] = "picotls";
ptls_iovec_t proposed_alpn[] = {
	{"grease", 6},
	{test_alpn, sizeof(test_alpn) - 1},
};

int on_extension_cb(ptls_on_extension_t* self, ptls_t* tls, uint8_t hstype, uint16_t exttype, ptls_iovec_t extdata) {
	return 0;
}

int should_collect_unknown_extension_cb(ptls_t* tls, struct st_ptls_handshake_properties_t* properties, uint16_t type) {
	return type == EXTENSION_2;
}

int on_client_hello_cb(ptls_on_client_hello_t* on_hello_cb_ctx, ptls_t* tls,
	ptls_on_client_hello_parameters_t* params) {
	int i;

	ptls_set_server_name(tls, params->server_name.base, params->server_name.len);

	for (i = 0; i < params->negotiated_protocols.count; i++) {
		if (params->negotiated_protocols.list[i].len == sizeof(test_alpn) - 1
			&& memcmp(params->negotiated_protocols.list[i].base, test_alpn, sizeof(test_alpn) - 1) == 0)
			ptls_set_negotiated_protocol(tls, params->negotiated_protocols.list[i].base, params->negotiated_protocols.list[i].len);
	}

	return 0;
}

int collected_unknown_extensions_cb(ptls_t* tls, ptls_handshake_properties_t* properties,
	ptls_raw_extension_t* slots) {
	ctx_t* ctx = (uint8_t*)properties - offsetof(ctx_t, properties);
	while (slots->type != UINT16_MAX) {
		if (slots->type == EXTENSION_2) {
			assert(slots->data.len <= sizeof(ctx->extension_data));
			ctx->extension_data_len = slots->data.len;
			memcpy(ctx->extension_data, slots->data.base, slots->data.len);
			if (ptls_is_server(tls)) {
				ctx->additional_extensions[0].type = EXTENSION_1;
				ctx->additional_extensions[0].data.base = extension_1_data;
				ctx->additional_extensions[0].data.len = sizeof(extension_1_data) - 1;

				ctx->additional_extensions[1].type = EXTENSION_2;
				ctx->additional_extensions[1].data.base = extension_2_server_data;
				ctx->additional_extensions[1].data.len = sizeof(extension_2_server_data) - 1;

				ctx->additional_extensions[2].type = UINT16_MAX;
				ctx->additional_extensions[2].data.base = NULL;
				ctx->additional_extensions[2].data.len = 0;

				properties->additional_extensions = ctx->additional_extensions;
			}
			break;
		}
		slots++;
	}
	return 0;
}

int handshake_test() {
	char* cert_file = "ec_cert.pem";
	char* key_file = "ec_key.pem";

	ptls_context_t client_tls_ctx = { 0 };
	ptls_t* client_tls;
	ptls_buffer_t client_buf;
	ctx_t client_ctx = { 0 };

	ptls_context_t server_tls_ctx = { 0 };
	ptls_t* server_tls;
	ptls_on_extension_t on_extension;
	ptls_on_client_hello_t on_client_hello;
	ptls_buffer_t server_buf;
	ctx_t server_ctx = { 0 };

	int ret;
	size_t len;

	on_extension.cb = on_extension_cb;

	client_tls_ctx.random_bytes = ptls_minicrypto_random_bytes;
	client_tls_ctx.get_time = &ptls_get_time;
	client_tls_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	client_tls_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	client_tls_ctx.verify_certificate = NULL;
	client_tls_ctx.on_extension = &on_extension;
	client_tls_ctx.use_exporter = 1;

	client_tls = ptls_client_new(&client_tls_ctx);
	ptls_set_server_name(client_tls, test_sni, sizeof(test_sni) - 1);

	client_ctx.additional_extensions[0].type = EXTENSION_1;
	client_ctx.additional_extensions[0].data.base = extension_1_data;
	client_ctx.additional_extensions[0].data.len = sizeof(extension_1_data) - 1;

	client_ctx.additional_extensions[1].type = EXTENSION_2;
	client_ctx.additional_extensions[1].data.base = extension_2_client_data;
	client_ctx.additional_extensions[1].data.len = sizeof(extension_2_client_data) - 1;

	client_ctx.additional_extensions[2].type = UINT16_MAX;
	client_ctx.additional_extensions[2].data.base = NULL;
	client_ctx.additional_extensions[2].data.len = 0;

	client_ctx.properties.additional_extensions = client_ctx.additional_extensions;
	client_ctx.properties.client.negotiated_protocols.list = proposed_alpn;
	client_ctx.properties.client.negotiated_protocols.count = sizeof(proposed_alpn) / sizeof(proposed_alpn[0]);
	client_ctx.properties.collect_extension = should_collect_unknown_extension_cb;
	client_ctx.properties.collected_extensions = collected_unknown_extensions_cb;

	server_tls_ctx.random_bytes = ptls_minicrypto_random_bytes;
	server_tls_ctx.get_time = &ptls_get_time;
	server_tls_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	server_tls_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	
	ret = ptls_load_certificates(&server_tls_ctx, cert_file);
	print_buf(server_tls_ctx.certificates.list[0].base, server_tls_ctx.certificates.list[0].len);
	
	ret = ptls_minicrypto_load_private_key(&server_tls_ctx, key_file);
	server_tls_ctx.on_extension = &on_extension;
	on_client_hello.cb = on_client_hello_cb;
	server_tls_ctx.on_client_hello = &on_client_hello;
	server_tls_ctx.use_exporter = 1;

	server_tls = ptls_server_new(&server_tls_ctx);

	server_ctx.properties.collect_extension = should_collect_unknown_extension_cb;
	server_ctx.properties.collected_extensions = collected_unknown_extensions_cb;

	ptls_buffer_init(&client_buf, "", 0);
	ptls_handshake(client_tls, &client_buf, NULL, NULL, &client_ctx.properties);

	ptls_buffer_init(&server_buf, "", 0);
	len = client_buf.off;
	ptls_handshake(server_tls, &server_buf, client_buf.base, &len, &server_ctx.properties);

	ptls_buffer_dispose(&client_buf);
	ptls_buffer_init(&client_buf, "", 0);
	len = server_buf.off;
	ptls_handshake(client_tls, &client_buf, server_buf.base, &len, &client_ctx.properties);

	ptls_buffer_dispose(&server_buf);
	ptls_buffer_init(&server_buf, "", 0);
	len = client_buf.off;
	ptls_handshake(server_tls, &server_buf, client_buf.base, &len, &server_ctx.properties);

	ptls_cipher_suite_t* client_cipher;
	uint8_t client_secret[64] = { 0 };
	char* label = "This is just a test";

	client_cipher = ptls_get_cipher(client_tls);
	assert(client_cipher != NULL);
	assert(client_cipher->hash->digest_size <= 64);
	ptls_export_secret(client_tls, client_secret, client_cipher->hash->digest_size, label, ptls_iovec_init(NULL, 0), 0);

	ptls_cipher_suite_t* server_cipher;
	uint8_t server_secret[64] = { 0 };

	server_cipher = ptls_get_cipher(server_tls);
	assert(server_cipher != NULL);
	assert(server_cipher->hash->digest_size <= 64);
	ptls_export_secret(server_tls, server_secret, server_cipher->hash->digest_size, label, ptls_iovec_init(NULL, 0), 0);

	assert(strcmp(client_cipher->aead->name, server_cipher->aead->name) == 0);
	assert(client_cipher->hash->digest_size == server_cipher->hash->digest_size);
	assert(memcmp(client_secret, server_secret, client_cipher->hash->digest_size) == 0);

	assert(ptls_get_server_name(server_tls) != NULL);
	assert(strcmp(ptls_get_server_name(client_tls), ptls_get_server_name(server_tls)) == 0);

	assert(ptls_get_negotiated_protocol(server_tls) != NULL);
	assert(strcmp(ptls_get_negotiated_protocol(server_tls), test_alpn) == 0);

	assert(server_ctx.extension_data_len == sizeof(extension_2_client_data) - 1);
	assert(memcmp(server_ctx.extension_data, extension_2_client_data, sizeof(extension_2_client_data) - 1) == 0);

	assert(client_ctx.extension_data_len == sizeof(extension_2_server_data) - 1);
	assert(memcmp(client_ctx.extension_data, extension_2_server_data, sizeof(extension_2_server_data) - 1) == 0);

	ptls_free(client_tls);

	for (size_t i = 0; i < server_tls_ctx.certificates.count; i++) {
		free(server_tls_ctx.certificates.list[i].base);
	}
	free(server_tls_ctx.certificates.list);

	free(server_tls_ctx.sign_certificate);

	ptls_free(server_tls);
}