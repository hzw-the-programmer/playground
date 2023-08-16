#include <stdio.h>
#include "mbedtls/net_sockets.h"

int main()
{
	mbedtls_net_context server_fd;

	mbedtls_net_init(&server_fd);
	printf("hello world");
	return 0;
}
