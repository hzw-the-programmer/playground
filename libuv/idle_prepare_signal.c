#include <stdlib.h>
#include "uv.h"

void check_cb(uv_check_t *check) {
    static int count = 0;
    printf("check: %d\n", count);
    count++;
    if (count > 15) {
        uv_check_stop(check);
    }
}

void signal_cb(uv_signal_t *signal, int signum) {
    static int count = 0;
    printf("signal: %d, signum=%d\n", count, signum);
    count++;
    if (count > 10) {
        uv_signal_stop(signal);
    }
}

void timer_cb(uv_timer_t *timer) {
    static int count = 0;
    printf("timer: %d\n", count);
    count++;
    if (count > 2) {
        uv_timer_stop(timer);
    }
}

void prepare_cb(uv_prepare_t *prepare) {
    static int count = 0;
    printf("prepare: %d\n", count);
    count++;
    if (count > 15) {
        uv_prepare_stop(prepare);
    }
}

void idle_cb(uv_idle_t *idle) {
    static int count = 0;
    printf("idle: %d\n", count);
    count++;
    if (count > 10) {
        uv_idle_stop(idle);
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    uv_check_t *check = malloc(sizeof(uv_check_t));
    uv_check_init(loop, check);
    uv_check_start(check, check_cb);

    uv_signal_t *signal = malloc(sizeof(uv_signal_t));
    uv_signal_init(loop, signal);
    uv_signal_start(signal, signal_cb, SIGINT);

    uv_timer_t *timer = malloc(sizeof(uv_timer_t));
    uv_timer_init(loop, timer);
    uv_timer_start(timer, timer_cb, 2000, 2000);

    uv_prepare_t *prepare = malloc(sizeof(uv_prepare_t));
    uv_prepare_init(loop, prepare);
    uv_prepare_start(prepare, prepare_cb);

    uv_idle_t *idle = malloc(sizeof(uv_idle_t));
    uv_idle_init(loop, idle);
    uv_idle_start(idle, idle_cb);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finish\n");

    free(check);
    free(signal);
    free(timer);
    free(prepare);
    free(idle);

    uv_loop_close(loop);
    free(loop);
    return 0;
}
