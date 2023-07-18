// clang-format -i client.c
// gcc client.c
// ./a.out

#include <netdb.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define SERVER_HOST "www.baidu.com"
#define SERVER_PORT 80
#define GET_REQUEST "GET / HTTP/1.1\r\n\r\n"

int main() {
  int server_fd, ret, len;
  struct hostent *server_host;
  struct sockaddr_in server_addr;
  unsigned char buf[1024];

  printf("\n  . Connecting to tcp/%s/%d", SERVER_HOST, SERVER_PORT);
  fflush(stdout);

  if ((server_host = gethostbyname(SERVER_HOST)) == NULL) {
    printf(" failed\n  ! gethostbyname failed\n\n");
    return 1;
  }

  if ((server_fd = socket(AF_INET, SOCK_STREAM, IPPROTO_IP)) < 0) {
    printf(" failed\n  ! socket returned %d\n\n", server_fd);
    return 1;
  }

  memcpy(&server_addr.sin_addr, server_host->h_addr, server_host->h_length);
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(SERVER_PORT);

  if ((ret = connect(server_fd, (struct sockaddr *)&server_addr,
                     sizeof(server_addr))) < 0) {
    printf(" failed\n  ! connect returned %d\n\n", ret);
    close(server_fd);
    return 1;
  }

  printf(" ok\n");

  printf("  > Write to server:");
  fflush(stdout);

  len = sprintf(buf, GET_REQUEST);
  while ((ret = write(server_fd, buf, len)) <= 0) {
    if (ret != 0) {
      printf(" failed\n write returned %d\n\n", ret);
      close(server_fd);
      return 1;
    }
  }

  len = ret;
  printf(" %d bytes written \n\n%s", len, buf);

  do {
    memset(buf, 0, sizeof(buf));
    printf("  < Read from server:");
    fflush(stdout);
    ret = read(server_fd, buf, sizeof(buf) - 1);
    if (ret <= 0) {
      printf(" failed\n  ! read returned %d\n\n", ret);
      break;
    }
    printf(" %d bytes read\n\n%s\n\n", ret, buf);
  } while (1);

  close(server_fd);
  return 0;
}
