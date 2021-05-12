#include <stdlib.h>
#include "uv.h"

void printtime() {
    time_t now = time(NULL);
    struct tm *tmp = localtime(&now);

    printf("%d-%d-%d %d:%d:%d\n",
        tmp->tm_year, tmp->tm_mon, tmp->tm_mday,
        tmp->tm_hour, tmp->tm_min, tmp->tm_sec);
}

void cb(uv_timer_t *timer) {
    static int count = 0;
    printtime();
    count++;
    if (count > 10) {
        uv_timer_stop(timer);
    }
}

int main(int argc, char *argv[]) {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    uv_timer_t *timer = malloc(sizeof(uv_timer_t));
    uv_timer_init(loop, timer);
    uv_timer_start(timer, cb, 2000, 1000);

    printtime();
    uv_run(loop, UV_RUN_DEFAULT);

    free(timer);

    uv_loop_close(loop);
    free(loop);
    return 0;
}