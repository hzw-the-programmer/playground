#include "picotls.h"
#include "minicrypto.h"

int main() {
	ptls_buffer_t buf;

	ptls_context_t ctx = { 0 };
	ctx.random_bytes = ptls_minicrypto_random_bytes;
	ctx.get_time = &ptls_get_time;
	ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	ctx.verify_certificate = NULL;

	ptls_t *tls = ptls_client_new(&ctx);
	char* host = "test.example.com";
	ptls_set_server_name(tls, host, strlen(host));
	ptls_buffer_init(&buf, "", 0);
	ptls_handshake_properties_t prop = { 0 };
	ptls_handshake(tls, &buf, NULL, NULL, &prop);
}