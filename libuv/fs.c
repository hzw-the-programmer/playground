
#include <stdio.h>
#include "uv.h"

uv_fs_t open_req;
uv_fs_t read_req;
uv_fs_t write_req;

uv_buf_t iov;
char buffer[1024];

void read_cb(uv_fs_t*);

void write_cb(uv_fs_t *req) {
    if (req->result < 0) {
        fprintf(stderr, "Write error: %s\n", uv_strerror(req->result));
    } else {
        uv_fs_read(uv_default_loop(), &read_req, open_req.result, &iov, 1, -1, read_cb);
    }
}

void read_cb(uv_fs_t *req) {
    printf("read_cb\n");

    if (req->result < 0) {
        fprintf(stderr, "Read error: %s\n", uv_strerror(req->result));
    } else if (req->result == 0) {

    } else {
        iov.len = req->result;
        uv_fs_write(uv_default_loop(), &write_req, open_req.result, &iov, 1, -1, write_cb);
    }
}

void open_cb(uv_fs_t *req) {
    printf("open_cb\n");

    if (req->result >= 0) {
        iov = uv_buf_init(buffer, sizeof(buffer));
        uv_fs_read(uv_default_loop(), &read_req, req->result, &iov, 1, -1, read_cb);
    } else {
        fprintf(stderr, "error opening file: %s\n", uv_strerror((int)req->result));
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = uv_default_loop();

    uv_fs_open(loop, &open_req, argv[1], O_RDONLY, 0, open_cb);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finish\n");
}
