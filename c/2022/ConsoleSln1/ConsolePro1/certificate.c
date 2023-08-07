#include "picotls.h"

void certificate_test() {
	ptls_context_t ctx = { 0 };
	size_t i;

	ptls_load_certificates(&ctx, "ec_cert.pem");
	for (i = 0; i < ctx.certificates.count; i++) {
		free(ctx.certificates.list[i].base);
	}
	free(ctx.certificates.list);
}