#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <assert.h>

#include "buffer.h"

//#define TEST

buffer_t* createBuffer(size_t size) {
    buffer_t *buf;

    buf = malloc(sizeof(buffer_t));
    if (buf == NULL) return NULL;

    buf->size = size;
    buf->write = 0;
    buf->read = 0;
    
    buf->data = malloc(size);
    if (buf->data == NULL) {
        free(buf);
        return NULL;
    }

    return buf;
}

void destroyBuffer(buffer_t *buf) {
    free(buf->data);
    free(buf);
}

uint8_t* getBufferWritePtr(buffer_t *buf) {
    return buf->data + buf->write;
}

size_t _getBufferWriteLen(buffer_t *buf) {
    size_t len;

    if (buf->write >= buf->read) {
        if (buf->read == 0) {
            len = buf->size - buf->write - 1;
        } else {
            len = buf->size - buf->write;
        }
    } else {
        len = buf->read - buf->write - 1;
    }

    return len;
}

void advanceBufferWrite(buffer_t *buf, size_t len) {
    buf->write = (buf->write + len) % buf->size;
}

size_t getBufferWriteLen(buffer_t *buf) {
    size_t total = 0;
    size_t len;
    size_t write;

    write = buf->write;

    while (len = _getBufferWriteLen(buf) != 0) {
        total += len;
        advanceBufferWrite(buf, len);
    }

    buf->write = write;

    return total;
}

size_t writeToBuffer(buffer_t *buf, uint8_t *data, size_t len) {
    size_t total = 0;
    size_t maxlen;

    while (maxlen = _getBufferWriteLen(buf) != 0) {
        if (len <= maxlen) {
            maxlen = len;
        }

        memcpy(buf->data + buf->write, data, maxlen);
        advanceBufferWrite(buf, maxlen);
        data += maxlen;
        len -= maxlen;
        total += maxlen;

        if (len == 0) {
            break;
        }
    }

    return total;
}

uint8_t* getBufferReadPtr(buffer_t *buf) {
    return buf->data + buf->read;
}

size_t _getBufferReadLen(buffer_t *buf) {
    size_t len;

    if (buf->write >= buf->read) {
        len = buf->write - buf->read;
    } else {
        len = buf->size - buf->read;
    }

    return len;
}

void advanceBufferRead(buffer_t *buf, size_t len) {
    buf->read = (buf->read + len) % buf->size;
}

size_t getBufferReadLen(buffer_t *buf) {
    size_t total = 0;
    size_t len;
    size_t read;

    read = buf->read;

    while (len = _getBufferReadLen(buf) != 0) {
        total += len;
        advanceBufferRead(buf, len);
    }

    buf->read = read;

    return total;
}

size_t readFromBuffer(buffer_t *buf, uint8_t *data, size_t len) {
    size_t total = 0;
    size_t maxlen;

    while (maxlen = _getBufferReadLen(buf) != 0) {
        if (len <= maxlen) {
            maxlen = len;
        }

        memcpy(data, buf->data + buf->read, maxlen);
        advanceBufferRead(buf, maxlen);
        data += maxlen;
        len -= maxlen;
        total += maxlen;

        if (len == 0) {
            break;
        }
    }

    return total;
}

void deAdvanceBufferRead(buffer_t *buf, size_t len) {
    size_t maxlen;

    if (buf->write >= buf->read) {
        maxlen = buf->read;
    } else {
        maxlen = buf->read - buf->write - 1;
    }

    if (len <= maxlen) {
        maxlen = len;
    }

    buf->read -= maxlen;
    len -= maxlen;

    if (len == 0 || (buf->write + 1) % buf->size == buf->read) {
        return;
    }

    buf->read = buf->size - 1;
    maxlen = buf->read - buf->write - 1;
    if (len <= maxlen) {
        maxlen = len;
    }

    buf->read -= maxlen;
    len -= maxlen;
}

#ifdef TEST

void testGetBufferWriteLen() {
    buffer_t *buf = createBuffer(10);
    
    assert(9 == getBufferWriteLen(buf));

    buf->write = 3;
    assert(6 == getBufferWriteLen(buf));

    buf->write = 5;
    assert(4 == getBufferWriteLen(buf));

    buf->write = 7;
    assert(2 == getBufferWriteLen(buf));

    buf->write = 9;
    assert(0 == getBufferWriteLen(buf));

    buf->write = 0;

    buf->read = 3;
    assert(2 == getBufferWriteLen(buf));

    buf->read = 5;
    assert(4 == getBufferWriteLen(buf));

    buf->read = 8;
    assert(7 == getBufferWriteLen(buf));

    buf->read = 9;
    assert(8 == getBufferWriteLen(buf));

    buf->write = 7;
    buf->read = 3;
    assert(5 == getBufferWriteLen(buf));

    buf->write = 3;
    buf->read = 7;
    assert(3 == getBufferWriteLen(buf));

    destroyBuffer(buf);
}

void testGetBufferReadLen() {
    buffer_t *buf = createBuffer(10);

    assert(0 == getBufferReadLen(buf));

    buf->write = 3;
    assert(3 == getBufferReadLen(buf));

    buf->write = 5;
    assert(5 == getBufferReadLen(buf));

    buf->write = 7;
    assert(7 == getBufferReadLen(buf));

    buf->write = 9;
    assert(9 == getBufferReadLen(buf));

    buf->write = 0;

    buf->read = 3;
    assert(7 == getBufferReadLen(buf));

    buf->read = 5;
    assert(5 == getBufferReadLen(buf));

    buf->read = 8;
    assert(2 == getBufferReadLen(buf));

    buf->read = 9;
    assert(1 == getBufferReadLen(buf));

    buf->write = 7;
    buf->read = 3;
    assert(4 == getBufferReadLen(buf));

    buf->write = 3;
    buf->read = 7;
    assert(6 == getBufferReadLen(buf));

    destroyBuffer(buf);
}

void testWriteToBuffer() {
    int len;
    buffer_t *buf = createBuffer(10);

    len = writeToBuffer(buf, "1234567890", 10);
    assert(9 == len);
    assert(strncmp("1234567890", buf->data, 9) == 0);

    buf->read = 3;
    len = writeToBuffer(buf, "1234567890", 10);
    assert(len == 3);
    assert(strncmp("2334567891", buf->data, 10) == 0);

    buf->read = 5;
    len = writeToBuffer(buf, "1234567890", 10);
    assert(len == 2);
    assert(strncmp("2312567891", buf->data, 10) == 0);

    buf->read = 8;
    len = writeToBuffer(buf, "1234567890", 10);
    assert(len == 3);
    assert(strncmp("2312123891", buf->data, 10) == 0);

    buf->read = 9;
    len = writeToBuffer(buf, "1234567890", 10);
    assert(len == 1);
    assert(strncmp("2312123191", buf->data, 10) == 0);

    buf->read = 0;
    len = writeToBuffer(buf, "1234567890", 10);
    assert(len == 1);
    assert(strncmp("2312123111", buf->data, 10) == 0);

    buf->read = 3;
    len = writeToBuffer(buf, "abcdefghij", 10);
    assert(len == 3);
    assert(strncmp("bc1212311a", buf->data, 10) == 0);

    buf->read = 2;
    len = writeToBuffer(buf, "abcdefghij", 10);
    assert(len == 9);
    assert(strncmp("icabcdefgh", buf->data, 10) == 0);

    buf->read = 7;
    len = writeToBuffer(buf, "abcdefghij", 10);
    assert(len == 5);
    assert(strncmp("iabcdeefgh", buf->data, 10) == 0);

    buf->read = 3;
    len = writeToBuffer(buf, "abcdefghij", 10);
    assert(len == 6);
    assert(strncmp("efbcdeabcd", buf->data, 10) == 0);
}

void testReadFromBuffer() {
    int len;
    char rbuf[20];
    buffer_t *buf = createBuffer(10);

    memcpy(buf->data, "abcdefghijklmn", 10);
    buf->write = 9;
    buf->read = 0;

    len = readFromBuffer(buf, rbuf, 20);
    assert(9 == len);
    assert(9 == buf->read);
    assert(strncmp("abcdefghijklmn", rbuf, len) == 0);

    memcpy(buf->data, "0123456789", 10);
    buf->write = 0;
    buf->read = 9;

    len = readFromBuffer(buf, rbuf, 20);
    assert(1 == len);
    assert(0 == buf->read);
    assert(strncmp("9bcdefghijklmn", rbuf, len) == 0);

    memcpy(buf->data, "9876543210", 10);
    buf->write = 3;
    buf->read = 7;

    len = readFromBuffer(buf, rbuf, 20);
    assert(6 == len);
    assert(3 == buf->read);
    assert(strncmp("210987", rbuf, len) == 0);

    memcpy(buf->data, "9876543210", 10);
    buf->write = 7;
    buf->read = 3;

    len = readFromBuffer(buf, rbuf, 20);
    assert(4 == len);
    assert(7 == buf->read);
    assert(strncmp("6543", rbuf, len) == 0);
}

int main(int argc, char *argv[]) {
    testGetBufferWriteLen();
    testGetBufferReadLen();
    testWriteToBuffer();
    testReadFromBuffer();
}

#endif
