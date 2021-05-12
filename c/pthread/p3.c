#include <stdio.h>
#include <unistd.h>
#include <pthread.h>

// gcc -pthread p3.c

void cleanup(void *arg) {
    printf("cleanup\n");
}

void* func(void *arg) {
    int oldstate;
    int ret;

    pthread_cleanup_push(cleanup, NULL);
    
    ret = pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, &oldstate);
    if (ret != 0) {
        printf("func: pthread_setcancelstate returns %d\n", ret);
    }
    printf("func: oldstate=%s\n", oldstate == PTHREAD_CANCEL_DISABLE ? "PTHREAD_CANCEL_DISABLE" : "PTHREAD_CANCEL_ENABLE");
    
    sleep(3);
    printf("func: after sleep\n");

    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL);
    sleep(3);
    printf("func: after sleep 1\n");

    pthread_cleanup_pop(1);
}
int main(int argc, char **argv) {
    pthread_t id;
    int ret;
    void *retval;

    ret = pthread_create(&id, NULL, func, NULL);
    if (ret != 0) {
        printf("pthread_create returns %d\n", ret);
    }

    sleep(1);

    ret = pthread_cancel(id);
    if (ret != 0) {
        printf("pthread_cancel returns %d\n", ret);
    }
    printf("after pthread_cancel\n");

    ret = pthread_join(id, &retval);
    if (ret != 0) {
        printf("pthread_join returns %d\n", ret);
    }

    if (retval == PTHREAD_CANCELED) {
        printf("retval: %s\n", "PTHREAD_CANCELED");
    }
}
