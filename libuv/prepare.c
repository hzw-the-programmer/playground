#include <stdlib.h>
#include "uv.h"

void cb(uv_prepare_t *prepare) {
    static int count = 0;
    printf("prepare %d\n", count);
    count++;
    if (count > 10) {
        uv_prepare_stop(prepare);
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    uv_prepare_t *prepare = malloc(sizeof(uv_prepare_t));
    uv_prepare_init(loop, prepare);
    uv_prepare_start(prepare, cb);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finish\n");

    uv_loop_close(loop);
    free(loop);
    return 0;
}
