#include "addressbook.pb-c.h"

void addressbook_process(size_t len, const uint8_t *buf) {
    AddressBook *b;

    b = address_book__unpack(NULL, len, buf);
}
