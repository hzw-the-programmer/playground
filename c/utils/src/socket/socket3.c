#define WIN32_LEAN_AND_MEAN

#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

#include "mem/mem.h"
#include "buffer/buffer.h"
#include "slice/slice.h"
#include "slice/split.h"


// Need to link with Ws2_32.lib, Mswsock.lib, and Advapi32.lib
#pragma comment (lib, "Ws2_32.lib")
#pragma comment (lib, "Mswsock.lib")
#pragma comment (lib, "AdvApi32.lib")

#define DEFAULT_SERVER "www.baidu.com"
#define DEFAULT_PORT "80"

#define DEFAULT_BUFLEN 512

typedef enum {
    DISCONNECTED,
    CONNECTING,
    IDLE,
    FIRSTLINE,
    HEADERS,
    BODY,
    TRAILER,
    WRITING,
} STATE_T;

#define CRNL "\r\n"

static void print_slice(FILE *f, const slice_t *s) {
#if 1
    assert(fwrite(s->data, 1, s->len, f) == s->len);
#else
    int i;

    for (i = 0; i < s->len; i++) {
        printf("%c", s->data[i]);
    }
#endif
}

void main(int argc, char **argv) 
{
    WSADATA wsa;
    int ret, len;
    struct addrinfo *result = NULL, *ptr = NULL, hints;
    SOCKET sock = INVALID_SOCKET;
    const char *req = "GET / HTTP/1.1\r\n\r\n";
    buf_t *send_buf;
    buf_t *buf;
    STATE_T state = FIRSTLINE;
    FILE *f;
    slice_t crnl = {CRNL, strlen(CRNL)};
    const char *server = DEFAULT_SERVER;
    const char *port = DEFAULT_PORT;
    int mode = 0;
    const char *fn;

    if (argc > 1) {
        server = argv[1];
    }
    if (argc > 2) {
        port = argv[2];
    }
    if (argc > 3) {
        mode = 1;
    }
    fn = server;
    if (argc > 4) {
        fn = argv[4];
    }
    

    // Initialize Winsock
    ret = WSAStartup(MAKEWORD(2,2), &wsa);
    if (ret != 0) {
        printf("WSAStartup failed with error: %d\n", ret);
        return;
    }

    ZeroMemory(&hints, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    // Resolve the server address and port
    ret = getaddrinfo(server, port, &hints, &result);
    if (ret != 0) {
        printf("getaddrinfo failed with error: %d\n", ret);
        WSACleanup();
        return;
    }

    // Attempt to connect to an address until one succeeds
    for(ptr=result; ptr != NULL ;ptr=ptr->ai_next) {
        // Create a SOCKET for connecting to server
        sock = socket(ptr->ai_family, ptr->ai_socktype,  ptr->ai_protocol);
        if (sock == INVALID_SOCKET) {
            printf("socket failed with error: %ld\n", WSAGetLastError());
            WSACleanup();
            return;
        }

        // Connect to server.
        ret = connect(sock, ptr->ai_addr, (int)ptr->ai_addrlen);
        if (ret == SOCKET_ERROR) {
            closesocket(sock);
            sock = INVALID_SOCKET;
            continue;
        }
        break;
    }

    freeaddrinfo(result);

    if (sock == INVALID_SOCKET) {
        printf("Unable to connect to server!\n");
        WSACleanup();
        return;
    }

    send_buf = buf_new(DEFAULT_BUFLEN);
    assert(send_buf);
    buf_write(send_buf, req, strlen(req));

    // Send an initial buffer
    ret = send(sock, buf_read_ptr(send_buf), buf_buffered(send_buf), 0);
    if (ret == SOCKET_ERROR) {
        printf("send failed with error: %d\n", WSAGetLastError());
        closesocket(sock);
        WSACleanup();
        return;
    }
    buf_read_inc(send_buf, ret);
    assert(!buf_buffered(send_buf));
    free(send_buf);

    printf("Bytes Sent: %ld\n", ret);

    // shutdown the connection since no more data will be sent
    ret = shutdown(sock, SD_SEND);
    if (ret == SOCKET_ERROR) {
        printf("shutdown failed with error: %d\n", WSAGetLastError());
        closesocket(sock);
        WSACleanup();
        return;
    }

    buf = buf_new(DEFAULT_BUFLEN);
    assert(buf);

    f = fopen(fn, "wb");
    assert(f);

    // Receive until the peer closes the connection
    do {
        len = recv(sock, buf_write_ptr(buf), buf_available(buf), 0);
        if (len > 0) {
            printf("Bytes received: %d\n", len);
            buf_write_inc(buf, len);

            if (mode) {
                assert(fwrite(buf_read_ptr(buf), 1, buf_buffered(buf), f) == buf_buffered(buf));
                buf_read_inc(buf, buf_buffered(buf));
                buf_tidy(buf);
            } else {
                if (state == FIRSTLINE || state == HEADERS) {
                    split_t split = split_new_ext(buf_read_ptr(buf), buf_buffered(buf), crnl.data, crnl.len);
                    while (1) {
                        slice_t line = split_next_ext(&split);
                        if (line.len != 0) {
                            if (state == FIRSTLINE) {
                                state = HEADERS;
                            }
                            print_slice(f, &line);
                            print_slice(f, &crnl);
                        } else {
                            if (line.data) {
                                state = BODY;
                                print_slice(f, &crnl);
                            }
                            break;
                        }
                    }
                    buf_read_inc(buf, buf_buffered(buf) - split.s.len);
                    buf_tidy(buf);
                } else if (state == BODY) {
                    slice_t s = slice_new(buf_read_ptr(buf), buf_buffered(buf));
                    print_slice(f, &s);
                    buf_read_inc(buf, buf_buffered(buf));
                    buf_tidy(buf);
                }
            }
        }
        else if (len == 0)
            printf("Connection closed\n");
        else
            printf("recv failed with error: %d\n", WSAGetLastError());
    } while(len > 0);

    free(buf);
    fclose(f);

    // cleanup
    closesocket(sock);
    WSACleanup();

    {
        char i;
        hmcheck();
        scanf("%c", &i);
    }
}
