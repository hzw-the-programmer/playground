#include <stdlib.h>
#include "uv.h"

void cb(uv_signal_t *signal, int signum) {
    static int count = 0;
    
    printf("signum=%d, %d\n", signal->signum, signum);

    count++;
    if (count > 2) {
        uv_signal_stop(signal);
    } else {
        if (signal->signum == SIGINT) {
            uv_signal_start(signal, cb, SIGTERM);
        } else {
            uv_signal_start(signal, cb, SIGINT);
        }
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    uv_signal_t *signal = malloc(sizeof(uv_signal_t));
    uv_signal_init(loop, signal);
    uv_signal_start(signal, cb, SIGINT);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finish\n");

    uv_loop_close(loop);
    free(loop);
    return 0;
}
