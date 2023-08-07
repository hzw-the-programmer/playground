#include <winsock2.h>
#include <stdio.h>
#include "picotls.h"
#include "minicrypto.h"

#define DEFAULT_BUFLEN 1024

int cli_test(int argc, char* argv[]) {
	WSADATA wsaData;
	int iResult;

	iResult = WSAStartup(MAKEWORD(2, 2), &wsaData);
	if (iResult != 0) {
		printf("WSAStartup failed: %d\n", iResult);
		return 1;
	}

	struct addrinfo hints, * result, *ptr;

	ZeroMemory(&hints, sizeof(hints));
	hints.ai_family = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;
	hints.ai_protocol = IPPROTO_TCP;

	iResult = getaddrinfo(argv[1], argv[2], &hints, &result);
	if (iResult != 0) {
		printf("getaddrinfo failed: %d\n", iResult);
		WSACleanup();
		return 1;
	}

	SOCKET ConnectSocket = INVALID_SOCKET;

	ptr = result;
	ConnectSocket = socket(ptr->ai_family, ptr->ai_socktype, ptr->ai_protocol);
	if (ConnectSocket == INVALID_SOCKET) {
		printf("Error at socket(): %d\n", WSAGetLastError());
		freeaddrinfo(result);
		WSACleanup();
		return 1;
	}

	iResult = connect(ConnectSocket, ptr->ai_addr, ptr->ai_addrlen);
	if (iResult == SOCKET_ERROR) {
		closesocket(ConnectSocket);
		ConnectSocket = INVALID_SOCKET;
	}

	freeaddrinfo(result);

	if (ConnectSocket == INVALID_SOCKET) {
		printf("Unable to connect to server!\n");
		WSACleanup();
		return 1;
	}

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

	closesocket(ConnectSocket);
	WSACleanup();

	return 0;
}