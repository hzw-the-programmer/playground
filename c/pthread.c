#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

// gcc pthread.c -pthread

struct event_arg {
    int a;
    int b;
};

void *event_loop(void* arg) {
    struct event_arg *_arg = arg;
    printf("event_loop. %d, %d\n", (*_arg).a, _arg->b);
    _arg->a++;
    _arg->b++;
    return arg;
}

void bye(void) {
    printf("Bye Bye!!!\n");
}

void bye2(int status, void *arg) {
    printf("Bye2 %d\n", status);
    struct event_arg *_arg = arg;
    printf("bye2. %d, %d\n", (*_arg).a, _arg->b);
}

int main(int argc, char *argv[]) {
    pthread_t thread;
    void *res;
    struct event_arg arg;

    arg.a = 1;
    arg.b = 2;

    atexit(bye);
    on_exit(bye2, &arg);
    printf("pthread_create begin.\n");
    pthread_create(&thread, NULL, event_loop, &arg);
    printf("pthread_create end.\n");
    
    pthread_join(thread, &res);
    struct event_arg *_arg = res;
    printf("join finished: %d, %d\n", _arg->a, _arg->b);
    //printf("join finished: %d, %d\n", arg.a, arg.b);
    return 123;
}
