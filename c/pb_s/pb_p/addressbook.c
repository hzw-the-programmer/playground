#include "addressbook.pb-c.h"

void print_book(AddressBook *book) {
    int i, j;

    if (!book) return;

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

void addressbook_process(size_t len, const uint8_t *buf) {
    AddressBook *book;

    book = address_book__unpack(NULL, len, buf);
    print_book(book);
    address_book__free_unpacked(book, NULL);

    {
        Person__PhoneNumber pn1 = {"111", PERSON__PHONE_TYPE__MOBILE};
        Person__PhoneNumber pn2 = {"2222", PERSON__PHONE_TYPE__HOME};
        Person__PhoneNumber pn3 = {"33333", PERSON__PHONE_TYPE__WORK};
        Person__PhoneNumber *pns[] = {&pn1, &pn2, &pn3};

        Person p1 = {"hzw", 0, "hzw@master.com", 3, pns};
        Person p2 = {"b1", 1, "b1@master.com", 3, pns};
        Person p3 = {"b2", 2, "b2@master.com", 3, pns};
        Person p4 = {"b3", 3, "b3@master.com", 3, pns};
        Person *ps[] = {&p1, &p2, &p3, &p4};

        AddressBook book1 = {4, ps};

        len = address_book__get_packed_size(&book1);
        buf = malloc(len);
        address_book__pack(&book1, buf);
        book = address_book__unpack(NULL, len, buf);
        print_book(book);
        address_book__free_unpacked(book, NULL);
    }
}
