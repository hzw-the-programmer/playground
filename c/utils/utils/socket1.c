#if 0
#define NUM_OF_SOCKETS 10

typedef struct {
    char buf[100];
} socket;

socket g_sockets[NUM_OF_SOCKETS] = {0};

int write(int fd, char *buf, int len) {

}

int read(int fd, char *buf, int len) {

}

void on_write(int fd) {
    int len;

    assert(fd < NUM_OF_SOCKETS);

    len = write(fd, buf, len);
}

void on_read(int fd) {

}

void test_socket() {
    int fd = 3;
    on_write(fd);
    on_write(fd);
}
#endif
