#include <winsock2.h>
#include <stdio.h>
#include "picotls.h"
#include "minicrypto.h"

#define DEFAULT_BUFLEN 1024

void process(SOCKET soc);

int cli_test(int argc, char* argv[]) {
	int err;

	WSADATA wsa;
	err = WSAStartup(MAKEWORD(2, 2), &wsa);
	if (err != 0) {
		printf("WSAStartup failed: %d\n", err);
		return 1;
	}

	struct addrinfo hints = {0}, * result = NULL, * ptr = NULL;
	hints.ai_family = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;
	hints.ai_protocol = IPPROTO_TCP;
	err = getaddrinfo(argv[1], argv[2], &hints, &result);
	if (err != 0) {
		printf("getaddrinfo failed: %d\n", err);
		WSACleanup();
		return 1;
	}

	SOCKET soc = INVALID_SOCKET;

	ptr = result;
	soc = socket(ptr->ai_family, ptr->ai_socktype, ptr->ai_protocol);
	if (soc == INVALID_SOCKET) {
		printf("Error at socket(): %d\n", WSAGetLastError());
		freeaddrinfo(result);
		WSACleanup();
		return 1;
	}

	err = connect(soc, ptr->ai_addr, ptr->ai_addrlen);
	if (err == SOCKET_ERROR) {
		freeaddrinfo(result);
		closesocket(soc);
		WSACleanup();
		return 1;
	}

	freeaddrinfo(result);

	process(soc);

	closesocket(soc);
	WSACleanup();

	return 0;

#if 0
	ptls_context_t tls_ctx = { 0 };
	ptls_t* tls;
	ptls_buffer_t sendbuf;

	tls_ctx.random_bytes = ptls_minicrypto_random_bytes;
	tls_ctx.get_time = &ptls_get_time;
	tls_ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	tls_ctx.cipher_suites = ptls_minicrypto_cipher_suites;
	//tls_ctx.verify_certificate = NULL;

	tls = ptls_client_new(&tls_ctx);

	ptls_buffer_init(&sendbuf, "", 0);
	ptls_handshake(tls, &sendbuf, NULL, 0, NULL);

	iResult = send(ConnectSocket, sendbuf.base, sendbuf.off, 0);
	if (iResult == SOCKET_ERROR) {
		printf("send failed: %d\n", WSAGetLastError());
		closesocket(ConnectSocket);
		WSACleanup();
		return 1;
	}

	if (iResult != 0) {
		if (iResult != sendbuf.off) {
			memmove(sendbuf.base, sendbuf.base + iResult, sendbuf.off - iResult);
		}
		sendbuf.off -= iResult;
	}

	printf("Bytes Sent: %d\n", iResult);

	/*
	iResult = shutdown(ConnectSocket, SD_SEND);
	if (iResult == SOCKET_ERROR) {
		printf("shutdown failed: %d\n", WSAGetLastError());
		closesocket(ConnectSocket);
		WSACleanup();
		return 1;
	}
	*/

	char recvBuf[DEFAULT_BUFLEN];
	int recvBufLen = DEFAULT_BUFLEN;

	size_t recvLen;
	do {
		recvLen = recv(ConnectSocket, recvBuf, recvBufLen, 0);
		if (recvLen > 0) {
			size_t len = recvLen;
			printf("Bytes received: %d\n", recvLen);
			if (ptls_handshake(tls, &sendbuf, recvBuf, &len, NULL) == 0) {
				iResult = send(ConnectSocket, sendbuf.base, sendbuf.off, 0);
				if (iResult == SOCKET_ERROR) {
					printf("send failed: %d\n", WSAGetLastError());
					closesocket(ConnectSocket);
					WSACleanup();
					return 1;
				}
				if (iResult != 0) {
					if (iResult != sendbuf.off) {
						memmove(sendbuf.base, sendbuf.base + iResult, sendbuf.off - iResult);
					}
					sendbuf.off -= iResult;
				}
				break;
			}
		}
		else if (recvLen == 0) {
			printf("Connection closed\n");
		}
		else {
			printf("recv failed: %d\n", WSAGetLastError());
		}
	} while (recvLen > 0);

	char* req = "GET / HTTP/1.1\r\n\r\n";
	ptls_send(tls, &sendbuf, req, strlen(req));
	iResult = send(ConnectSocket, sendbuf.base, sendbuf.off, 0);
	if (iResult == SOCKET_ERROR) {
		printf("send failed: %d\n", WSAGetLastError());
		closesocket(ConnectSocket);
		WSACleanup();
		return 1;
	}
	if (iResult != 0) {
		if (iResult != sendbuf.off) {
			memmove(sendbuf.base, sendbuf.base + iResult, sendbuf.off - iResult);
		}
		sendbuf.off -= iResult;
	}

	ptls_buffer_t recvbuf;
	ptls_buffer_init(&recvbuf, "", 0);

	do {
		recvLen = recv(ConnectSocket, recvBuf, recvBufLen, 0);
		if (recvLen > 0) {
			size_t len = recvLen;
			printf("Bytes received: %d\n", recvLen);
			ptls_receive(tls, &recvbuf, recvBuf, &recvLen);
		}
		else if (recvLen == 0) {
			printf("Connection closed\n");
		}
		else {
			printf("recv failed: %d\n", WSAGetLastError());
		}
	} while (recvLen > 0);
#endif
}

void buf_shift(ptls_buffer_t *buf, size_t delta) {
	if (delta < buf->off) {
		memmove(buf->base, buf->base + delta, buf->off - delta);
	}
	buf->off -= delta;
}

void process(SOCKET soc) {
	ptls_context_t ctx = { 0 };
	ctx.random_bytes = ptls_minicrypto_random_bytes;
	ctx.get_time = &ptls_get_time;
	ctx.key_exchanges = ptls_minicrypto_key_exchanges;
	ctx.cipher_suites = ptls_minicrypto_cipher_suites;

	ptls_t *tls = ptls_client_new(&ctx);

	ptls_buffer_t sendbuf;
	ptls_buffer_init(&sendbuf, "", 0);
	ptls_handshake(tls, &sendbuf, NULL, 0, NULL);

	ptls_buffer_t recvbuf;
	ptls_buffer_init(&recvbuf, "", 0);

	while (1) {
		int result;
		
		while (sendbuf.off) {
			result = send(soc, sendbuf.base, sendbuf.off, 0);
			if (result > 0) {
				buf_shift(&sendbuf, result);
			}
			else {
				goto end;
			}
		}

		uint8_t buf[DEFAULT_BUFLEN];

		result = recv(soc, buf, DEFAULT_BUFLEN, 0);
		if (result > 0) {
			size_t len = result, left = 0, off = 0;
			while ((left = len - off) > 0) {
				if (!ptls_handshake_is_complete(tls)) {
					result = ptls_handshake(tls, &sendbuf, buf, &left, NULL);
					if (result == 0) {
						uint8_t req[] = "GET / HTTP/1.1\r\n\r\n";
						ptls_send(tls, &sendbuf, req, sizeof(req) - 1);
					}
				}
				else {
					result = ptls_receive(tls, &recvbuf, buf, &left);
					if (result == 0) {
						recvbuf.off = 0;
					}
				}
				off += left;
			}
		}
		else {
			goto end;
		}
	}

end:
	ptls_buffer_dispose(&sendbuf);
	ptls_buffer_dispose(&recvbuf);
	ptls_free(tls);
}