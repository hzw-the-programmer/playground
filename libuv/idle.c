#include <stdlib.h>
#include "uv.h"

void cb(uv_idle_t *idle) {
    static int count = 1;

    for (int i = 0; i < count; i++) {
        printf("*");
    }
    printf("\n");

    count++;

    if (count > 10) {
        uv_idle_stop(idle);
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    uv_idle_t *idle = malloc(sizeof(uv_idle_t));
    uv_idle_init(loop, idle);
    uv_idle_start(idle, cb);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finish\n");

    free(idle);

    uv_loop_close(loop);
    free(loop);
    return 0;
}