#include "stdint.h"

typedef struct {
    int32_t id;
    char *name;
} PersonM;

typedef struct {
    PersonM **people;
} AddressBookM;
