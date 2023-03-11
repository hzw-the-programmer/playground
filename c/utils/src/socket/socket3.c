#define WIN32_LEAN_AND_MEAN

#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <stdlib.h>
#include <stdio.h>

#include "slice.h"
#include "split.h"


// Need to link with Ws2_32.lib, Mswsock.lib, and Advapi32.lib
#pragma comment (lib, "Ws2_32.lib")
#pragma comment (lib, "Mswsock.lib")
#pragma comment (lib, "AdvApi32.lib")

#define DEFAULT_SERVER "www.baidu.com"
#define DEFAULT_PORT "80"

#define DEFAULT_BUFLEN 512

typedef struct {
    int cap;
    int len;
    unsigned char *data;
} buf_t;

void main() 
{
    WSADATA wsaData;
    int iResult;
    struct addrinfo *result = NULL, *ptr = NULL, hints;
    SOCKET ConnectSocket = INVALID_SOCKET;
    const char *sendbuf = "GET / HTTP/1.1\r\n\r\n";
    char data[DEFAULT_BUFLEN];
    buf_t buf = {DEFAULT_BUFLEN, 0, data};
    int len;

    // Initialize Winsock
    iResult = WSAStartup(MAKEWORD(2,2), &wsaData);
    if (iResult != 0) {
        printf("WSAStartup failed with error: %d\n", iResult);
        return;
    }

    ZeroMemory(&hints, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    // Resolve the server address and port
    iResult = getaddrinfo(DEFAULT_SERVER, DEFAULT_PORT, &hints, &result);
    if ( iResult != 0 ) {
        printf("getaddrinfo failed with error: %d\n", iResult);
        WSACleanup();
        return;
    }

    // Attempt to connect to an address until one succeeds
    for(ptr=result; ptr != NULL ;ptr=ptr->ai_next) {
        // Create a SOCKET for connecting to server
        ConnectSocket = socket(ptr->ai_family, ptr->ai_socktype,  ptr->ai_protocol);
        if (ConnectSocket == INVALID_SOCKET) {
            printf("socket failed with error: %ld\n", WSAGetLastError());
            WSACleanup();
            return;
        }

        // Connect to server.
        iResult = connect(ConnectSocket, ptr->ai_addr, (int)ptr->ai_addrlen);
        if (iResult == SOCKET_ERROR) {
            closesocket(ConnectSocket);
            ConnectSocket = INVALID_SOCKET;
            continue;
        }
        break;
    }

    freeaddrinfo(result);

    if (ConnectSocket == INVALID_SOCKET) {
        printf("Unable to connect to server!\n");
        WSACleanup();
        return;
    }

    // Send an initial buffer
    iResult = send(ConnectSocket, sendbuf, (int)strlen(sendbuf), 0);
    if (iResult == SOCKET_ERROR) {
        printf("send failed with error: %d\n", WSAGetLastError());
        closesocket(ConnectSocket);
        WSACleanup();
        return;
    }

    printf("Bytes Sent: %ld\n", iResult);

    // shutdown the connection since no more data will be sent
    iResult = shutdown(ConnectSocket, SD_SEND);
    if (iResult == SOCKET_ERROR) {
        printf("shutdown failed with error: %d\n", WSAGetLastError());
        closesocket(ConnectSocket);
        WSACleanup();
        return;
    }

    // Receive until the peer closes the connection
    do {
        len = recv(ConnectSocket, buf.data + buf.len, buf.cap - buf.len, 0);
        if (len > 0) {
            split_t lines;
            
            printf("Bytes received: %d\n", len);

            lines = split_new(slice_new(buf.data, buf.len + len), '\n');
            while (1) {
                slice_t line = split_next(&lines);
                if (line.len == 0) {
                    break;
                }
                if (line.data[line.len] == '\n') {
                    line.data[line.len] = 0;
                    printf("%s\n", line.data);
                    line.data[line.len] = '\n';
                } else {
                    memmove(buf.data, line.data, line.len);
                    buf.len = line.len;
                }
            }
        }
        else if (len == 0)
            printf("Connection closed\n");
        else
            printf("recv failed with error: %d\n", WSAGetLastError());
    } while(len > 0);

    // cleanup
    closesocket(ConnectSocket);
    WSACleanup();

    {
        char i;
        scanf("%c", &i);
    }
}
