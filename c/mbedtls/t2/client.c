#include "mbedtls/ctr_drbg.h"
#include "mbedtls/net_sockets.h"
#include <string.h>

#define SERVER_NAME "www.baidu.com"
#define SERVER_PORT "443"
#define GET_REQUEST "GET / HTTP/1.1\r\n\r\n"

static void my_debug(void *ctx, int level, const char *file, int line,
                     const char *str) {
  fprintf((FILE *)ctx, "%s:%04d: %s", file, line, str);
  fflush((FILE *)ctx);
}

int main() {
  int ret, len;
  mbedtls_ssl_config config;
  mbedtls_ctr_drbg_context ctr_drbg;
  mbedtls_entropy_context entropy;
  char *pers = "ssl_ciient1";
  mbedtls_net_context server_fd;
  mbedtls_ssl_context ssl;
  unsigned char buf[1024];

  mbedtls_ssl_config_init(&config);

  if ((ret = mbedtls_ssl_config_defaults(&config, MBEDTLS_SSL_IS_CLIENT,
                                         MBEDTLS_SSL_TRANSPORT_STREAM,
                                         MBEDTLS_SSL_PRESET_DEFAULT)) != 0) {
    printf(" failed\n  ! mbedtls_ssl_config_defaults returned %d \n\n", ret);
    return 1;
  }

  mbedtls_ssl_conf_authmode(&config, MBEDTLS_SSL_VERIFY_NONE);

  mbedtls_ctr_drbg_init(&ctr_drbg);
  mbedtls_entropy_init(&entropy);
  if ((ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                                   pers, strlen(pers))) != 0) {
    printf(" failed\n  ! mbedtls_ctr_drbg_seed returned %d\n\n", ret);
    return 1;
  }

  mbedtls_ssl_conf_rng(&config, mbedtls_ctr_drbg_random, &ctr_drbg);
  mbedtls_ssl_conf_dbg(&config, my_debug, stdout);

  mbedtls_ssl_init(&ssl);
  mbedtls_ssl_setup(&ssl, &config);
  if ((ret = mbedtls_ssl_set_hostname(&ssl, SERVER_NAME)) != 0) {
    printf(" failed\n  ! mbedtls_ssl_set_hostname returned %d\n\n", ret);
    return 1;
  }

  mbedtls_net_init(&server_fd);
  if ((ret = mbedtls_net_connect(&server_fd, SERVER_NAME, SERVER_PORT,
                                 MBEDTLS_NET_PROTO_TCP)) != 0) {
    printf(" failed\n  ! mbedtls_net_connect returned %d\n\n", ret);
    return 1;
  }

  mbedtls_ssl_set_bio(&ssl, &server_fd, mbedtls_net_send, mbedtls_net_recv,
                      NULL);

  len = sprintf(buf, GET_REQUEST);
  while ((ret = mbedtls_ssl_write(&ssl, buf, len)) <= 0) {
    if (ret != 0) {
      printf(" failed\n ! mbedtls_ssl_write returned -%#x\n\n", -ret);
      break;
    }
  }

  do {
    memset(buf, 0, sizeof(buf));
    printf("  < Reading from server:");
    fflush(stdout);
    ret = mbedtls_ssl_read(&ssl, buf, sizeof(buf) - 1);
    if (ret <= 0) {
      printf(" failed\n  ! mbedtls_ssl_read returned -%#x\n\n", -ret);
      break;
    }
    printf(" %d bytes read\n\n%s\n\n", ret, buf);
  } while (1);
}
