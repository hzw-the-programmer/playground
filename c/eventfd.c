#include <sys/eventfd.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

///proc/[pid]/fdinfo/[efd]

#define handle_error(msg) \
    do { perror(msg); exit(EXIT_FAILURE); } while (0)

int
main(int argc, char *argv[])
{
    int efd, j;
    uint64_t u;
    ssize_t s;

    if (argc < 2) {
        fprintf(stderr, "Usage: %s <num>...\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    printf("pid=%d\n", getpid());

    efd = eventfd(0, 0);
    if (efd == -1)
        handle_error("eventfd");
    printf("efd=%d\n", efd);

    switch (fork()) {
    case 0:
        printf("pid=%d\n", getpid());
        for (j = 1; j < argc; j++) {
            printf("Child writing %s to efd\n", argv[j]);
            u = strtoull(argv[j], NULL, 0);
            s = write(efd, &u, sizeof(uint64_t));
            if (s != sizeof(uint64_t))
                handle_error("write");
        }
        printf("Child completed write loop\n");
        sleep(20);

        exit(EXIT_SUCCESS);

    default:
        sleep(20);

        printf("Parent about to read\n");
        s = read(efd, &u, sizeof(uint64_t));
        if (s != sizeof(uint64_t))
            handle_error("read");
        printf("Parent read %llu (0x%llx) from efd\n",
            (unsigned long long) u, (unsigned long long) u);
        exit(EXIT_FAILURE);

    case -1:
        handle_error("fork");
    }
}
