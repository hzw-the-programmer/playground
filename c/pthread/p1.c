#include <stdio.h> // printf
#include <unistd.h> // sleep
#include <pthread.h>

// gcc -pthread p1.c

void cleanup(void *arg) {
    printf("cleanup: %ld\n", (long)arg);
}

void* func(void *arg) {
    long i = (long)arg;

    pthread_cleanup_push(cleanup, (void*)i);    

    printf("func: i=%ld\n", i);
    pthread_exit((void*)2311);
    pthread_cleanup_pop(1);
    printf("func after cleanup pop\n");
    return (void*)2311;
}

int main(int argc, char **argv) {
    pthread_t t;
    int ret;
    long i;

    printf("%ld %ld %ld\n", sizeof(int), sizeof(long), sizeof(void*));

    i = 1123;
    ret = pthread_create(&t, NULL, func, (void*)i);
    if (ret != 0) {
        printf("error1");
    }

    sleep(3);

    ret = pthread_join(t, (void**)&i);
    if (ret != 0) {
        printf("error2");
    }
    printf("main: i=%ld\n", i);
}
