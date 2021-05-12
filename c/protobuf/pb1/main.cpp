#include <stdio.h>
#include <stdlib.h>
#include "stdint.h"
#include "addressbook.pb-c.h"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s fn\n", argv[0]);
        return 1;
    }
    char *fn = argv[1];

    FILE *f = fopen(fn, "rb");
    if (!f) {
        perror("fopen");
        return 1;
    }

    if (fseek(f, 0, SEEK_END)) {
        perror("fseek");
        return 1;
    }

    long size = ftell(f);
    if (size == -1) {
        perror("ftell");
        return 1;
    }

    if (fseek(f, 0, SEEK_SET)) {
        perror("fseek");
        return 1;
    }

    uint8_t *data = (uint8_t*)malloc(size);
    if (!data) {
        perror("malloc");
        return 1;
    }

    size_t n = fread(data, 1, size, f);
    if (n != size) {
        printf("fread: %d!=%d", n, size);
        return 1;
    }
 
    AddressBook *book = address_book__unpack(NULL, size, data);

    for (int i = 0; i < book->n_people; i++) {
        Person *p = book->people[i];
        printf("Id: %d\n", p->id);
        printf("Name: %s\n", p->name);
        printf("Email: %s\n", p->email);
        for (int j = 0; j < p->n_phones; j++) {
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