#include <stdio.h>
#include <unistd.h>
#include <pthread.h>

void* func(void *arg) {
    printf("func\n");
    sleep(3);
    printf("func after sleep\n");
}

int main(char **argv) {
    pthread_t id;
    int ret;

    ret = pthread_create(&id, NULL, func, NULL);
    if (ret != 0) {
        printf("error1");
    }
    sleep(1);
}