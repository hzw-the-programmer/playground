#include <stdio.h>
#include <stdlib.h>
#include "stdint.h"
#include "addressbook.pb-c.h"

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

int main(int argc, char *argv[]) {
    uint8_t *data;
    long fs;
    AddressBook *book;
    int i, j;
    
    if (argc != 2) {
        printf("Usage: %s fn\n", argv[0]);
        return 1;
    }

    if (get_content(argv[1], &data, &fs)) {
        printf("get_content failed\n");
        return 1;
    }
 
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

    return 0;
}
