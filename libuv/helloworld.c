#include <stdio.h>
#include <stdlib.h>
#include <uv.h>

//gcc helloworld.c -I../../libuv/include -L../../libuv/.libs -luv
//LD_LIBRARY_PATH=/home/zhiwenhe/work/libuv/.libs:$LID_LIBRARY_PATH ./a.out

int main() {
    uv_loop_t *loop = malloc(sizeof(uv_loop_t));
    uv_loop_init(loop);

    printf("run loop\n");
    uv_run(loop, UV_RUN_DEFAULT);
    printf("loop finished\n");

    uv_loop_close(loop);
    free(loop);
    return 0;
}