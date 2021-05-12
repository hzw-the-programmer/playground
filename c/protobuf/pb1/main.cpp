#include <stdio.h>
#include "stdint.h"
#include "addressbook.pb-c.h"

int main() {
    uint8_t data[] = {0x0a, 0x21, 0x0a, 0x04, 0x62, 0x6a, 0x30, 0x31, 0x10, 0x01, 0x1a, 0x0c, 0x62, 0x6a, 0x30, 0x31, 0x40, 0x68, 0x7a, 0x77, 0x2e, 0x63, 0x6f, 0x6d, 0x22, 0x09, 0x0a, 0x07, 0x31, 0x32, 0x33, 0x34, 0x31, 0x36, 0x36};
    AddressBook *book = address_book__unpack(NULL, sizeof(data), data);
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
    int i;
    scanf("%d\n", &i);
    return 0;
}