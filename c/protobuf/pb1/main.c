#include <stdio.h>
#include <stdlib.h>
#include "stdint.h"
// #include "addressbook.pb-c.h"
// #include "test1.pb-c.h"
#include "pb_encode.h"
#include "pb_decode.h"
#include "simple.pb.h"
#include "addressbook.pb.h"

int get_content(const char *fn, uint8_t **data, long *fs) {
    FILE *f;
    size_t n;
    int result = 1;

    f = fopen(fn, "rb");
    if (!f) {
        perror("fopen");
        goto end;
    }

    if (fseek(f, 0, SEEK_END)) {
        perror("fseek");
        goto end;
    }

    *fs = ftell(f);
    if (*fs == -1) {
        perror("ftell");
        goto end;
    }

    if (fseek(f, 0, SEEK_SET)) {
        perror("fseek");
        goto end;
    }

    *data = (uint8_t*)malloc(*fs);
    if (!*data) {
        perror("malloc");
        goto end;
    }

    n = fread(*data, 1, *fs, f);
    if (n != *fs) {
        free(*data);
        printf("fread: %d!=%d", n, *fs);
        goto end;
    }

    result = 0;

end:
    if (f) {
        fclose(f);
    }
    return result;
}

/*
void address_book(long fs, uint8_t *data) {
    AddressBook *book;
    int i, j;

    book = address_book__unpack(NULL, fs, data);

    for (i = 0; i < book->n_people; i++) {
        Person *p = book->people[i];
        printf("Id: %d\n", p->id);
        printf("Name: %s\n", p->name);
        printf("Email: %s\n", p->email);
        for (j = 0; j < p->n_phones; j++) {
            Person__PhoneNumber *pn = p->phones[j];
            switch (pn->type) {
            case PERSON__PHONE_TYPE__MOBILE:
                printf("Mobile phone #: %s\n", pn->number);
                break;
            case PERSON__PHONE_TYPE__HOME:
                printf("Home phone #: %s\n", pn->number);
                break;
            case PERSON__PHONE_TYPE__WORK:
                printf("Work phone #: %s\n", pn->number);
                break;
            }
        }
    }
}

void t1(long fs, uint8_t *data) {
    Test1 *t1;

    t1 = test1__unpack(NULL, fs, data);
    printf("F1: %s\n", t1->f1);
    printf("F2: %s\n", t1->f2);
    printf("F3: %s\n", t1->f3);
    printf("F4: %s\n", t1->f4);
    printf("F5: %s\n", t1->f5);
    printf("F6: %d\n", t1->f6);
    printf("F7: %d\n", t1->f7);
    printf("F8: %s\n", t1->f8);
    printf("F9: %s\n", t1->f9);
}
*/

void simple() {
    uint8_t *buf;
    size_t bufsize;
    pb_ostream_t ostream;
    pb_istream_t istream;
    SimpleMessage message = SimpleMessage_init_zero;
    bool status;
    
    message.lucky_number = 13;
    pb_get_encoded_size(&bufsize, SimpleMessage_fields, &message);

    buf = (uint8_t*)malloc(bufsize);
    ostream = pb_ostream_from_buffer(buf, bufsize);
    status = pb_encode(&ostream, SimpleMessage_fields, &message);

    istream = pb_istream_from_buffer(buf, bufsize);
    message.lucky_number = 0;
    status = pb_decode(&istream, SimpleMessage_fields, &message);
}

bool decode_people(pb_istream_t *stream, const pb_field_t *field, void **arg) {
    printf("haha");
}

void address_book(long fs, uint8_t *data) {
    AddressBook book = AddressBook_init_zero;
    pb_istream_t istream;
    bool status;
    
    book.people.funcs.decode = decode_people;

    istream = pb_istream_from_buffer(data, fs);
    status = pb_decode(&istream, AddressBook_fields, &book);
}

int main(int argc, char *argv[]) {
    uint8_t *data;
    long fs;
    
    if (argc != 2) {
        printf("Usage: %s fn\n", argv[0]);
        return 1;
    }

    if (get_content(argv[1], &data, &fs)) {
        printf("get_content failed\n");
        return 1;
    }
 
    address_book(fs, data);
    // t1(fs, data);
    simple();

    return 0;
}
