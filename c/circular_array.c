#include <assert.h>
#include <string.h>

typedef char elem_t;

typedef struct circular_array {
    elem_t *buf;
    int len;
    int write;
    int read;
} circular_array_t;

void circular_array_available(circular_array_t *ca, int *wa, int *ra) {
    if (ca->read > ca->write) {
        *wa = ca->read - ca->write - 1;
        *ra = ca->len - 1 - *wa;
    } else {
        *ra = ca->write - ca->read;
        *wa = ca->len - 1 - *ra;
    }
}

void circular_array_init(circular_array_t *ca, elem_t *buf, int len) {
    ca->buf = buf;
    ca->len = len;
    ca->write = 0;
    ca->read = 0;
}

int circular_array_write(circular_array_t *ca, elem_t *buf, int len) {
    return 1;
}

int circular_array_read(circular_array_t *ca, elem_t *buf, int len) {
    return 1;
}

int main(int argc, char *argv[]) {
    elem_t buf[10];
    elem_t res[10];
    int wlen, rlen;
    circular_array_t ca;

    int wa, ra;
    ca.len = 10;

    for (int i = 0; i < 10; i++) {
        ca.write = i;
        ca.read = i;
        circular_array_available(&ca, &wa, &ra);
        assert(9 == wa);
        assert(0 == ra);
    }

    ca.read = 0;
    ca.write = 9;
    circular_array_available(&ca, &wa, &ra);
    assert(0 == wa);
    assert(9 == ra);

    ca.write = 0;
    ca.read = 9;
    circular_array_available(&ca, &wa, &ra);
    assert(8 == wa);
    assert(1 == ra);

    //circular_array_init(&ca, buf, sizeof(buf));
    //wlen = circular_array_write(&ca, "abc", 3);
    //assert(wlen == 3);
    //rlen = circular_array_read(&ca, res, sizeof(res));
    //assert(rlen == 3);
    //res[rlen] = '\0';
    //assert(strcmp("abc", res) == 0);
}
