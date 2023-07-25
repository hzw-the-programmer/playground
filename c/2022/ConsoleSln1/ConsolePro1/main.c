#include "picotls.h"
#include "minicrypto.h"

int on_extension_cb(ptls_on_extension_t* self, ptls_t* tls, uint8_t hstype, uint16_t exttype, ptls_iovec_t extdata) {
	return 0;
}

int collect_extension_cb(ptls_t* tls, struct st_ptls_handshake_properties_t* properties, uint16_t type) {

}

int on_client_hello_cb(ptls_on_client_hello_t* on_hello_cb_ctx, ptls_t* tls,
	ptls_on_client_hello_parameters_t* params) {

}

int main() {
	char* host = "test.example.com";
	char* cert_file = "ec_cert.pem";
	char* key_file = "ec_key.pem";

	ptls_context_t client_ctx = { 0 };
	ptls_t* client_tls;
	ptls_handshake_properties_t client_prop = { 0 };
	ptls_buffer_t client_buf;

	ptls_context_t server_ctx = { 0 };
	ptls_on_extension_t on_extension;
	ptls_on_client_hello_t on_client_hello;
	ptls_handshake_properties_t server_prop = { 0 };
	ptls_t* server_tls;
	ptls_buffer_t server_buf;

	int ret;
	size_t len;

	client_ctx.random_bytes = ptls_minicrypto_random_bytes;
	client_ctx.get_time = &ptls_get_time;
	client_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	client_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	client_ctx.verify_certificate = NULL;

	client_tls = ptls_client_new(&client_ctx);
	ptls_set_server_name(client_tls, host, strlen(host));

	server_ctx.random_bytes = ptls_minicrypto_random_bytes;
	server_ctx.get_time = &ptls_get_time;
	server_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	server_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	ret = ptls_load_certificates(&server_ctx, cert_file);
	ret = ptls_minicrypto_load_private_key(&server_ctx, key_file);
	on_extension.cb = on_extension_cb;
	on_client_hello.cb = on_client_hello_cb;
	server_ctx.on_extension = &on_extension;
	server_ctx.on_client_hello = &on_client_hello;

	server_prop.collect_extension = collect_extension_cb;

	server_tls = ptls_server_new(&server_ctx);
	
	ptls_buffer_init(&client_buf, "", 0);
	ptls_handshake(client_tls, &client_buf, NULL, NULL, &client_prop);

	ptls_buffer_init(&server_buf, "", 0);
	len = client_buf.off;
	ptls_handshake(server_tls, &server_buf, client_buf.base, &len, &server_prop);
}