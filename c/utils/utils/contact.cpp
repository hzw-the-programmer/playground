#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "memory.h"

typedef struct {
    char *Name;
    long long Phone;
} Contact;

void ReverseEndian(char *in, int inLen, char *out) {
    for (int i = 0; i < inLen; i++) {
        out[i] = in[inLen - 1 - i];
    }
}

int PackContact(Contact *contact, char *buf, int bufLen) {
    if (contact == 0) {
        return -1;
    }
    
    int nameLen = strlen(contact->Name);
    int totalLen = 8 + 1 + nameLen;

    if (bufLen >= totalLen) {
        char *p = (char*)&contact->Phone;
        ReverseEndian(p, 8, buf);
        buf += 8;
        *buf = nameLen;
        buf += 1;
        memcpy(buf, contact->Name, nameLen);
    }

    return totalLen;
}

int UnpackContact(Contact *contact, char *buf, int bufLen) {
    if (bufLen < 8 + 1) {
        return -1;
    }

    if (contact) {
        ReverseEndian(buf, 8, (char*)&contact->Phone);
    }
    buf += 8;
    bufLen -= 8;
    
    int nameLen = buf[0];
    buf += 1;
    bufLen -= 1;
    
    if (bufLen < nameLen) {
        return -1;
    }

    if (contact) {
        contact->Name = (char*)HZW_MALLOC(nameLen + 1);
        memcpy(contact->Name, buf, nameLen);
        contact->Name[nameLen] = 0;
    }

    return 8 + 1 + nameLen;
}

int PackContacts(Contact *contacts, int contactsLen, char *buf, int bufLen) {
    int totalLen = 0;
    
    if (bufLen == 0) {
        for (int i = 0; i < contactsLen; i++) {
            int len = PackContact(contacts+i, 0, 0);
            if (len == -1) {
                return -1;
            }
            totalLen += len;
        }
        return totalLen;
    }

    for (int i = 0; i < contactsLen; i++) {
        int len = PackContact(contacts+i, buf, bufLen);
        if (len == -1) {
            return -1;
        }
        buf += len;
        bufLen -= len;
        totalLen += len;
    }

    return totalLen;
}

int UnpackContacts(Contact **contacts, int *contactsLen, char *buf, int bufLen) {
    char *b;
    int bl;

    b = buf;
    bl = bufLen;
    *contactsLen = 0;
    while (1) {
        int l = UnpackContact(0, b, bl);
        if (l == -1) {
            break;
        }
        b += l;
        bl -= l;
        *contactsLen += 1;
    }

    b = buf;
    bl = bufLen;
    *contacts = (Contact*)HZW_MALLOC(*contactsLen * sizeof(Contact));
    for (int i = 0; i < *contactsLen; i++) {
        int l = UnpackContact((*contacts)+i, b, bl);
        if (l == -1) {
            return -1;
        }
        b += l;
        bl -= l;
    }

    return bufLen - bl;
}

void testContact() {
    printf("sizeof(int)=%d\n", sizeof(int));
    printf("sizeof(long)=%d\n", sizeof(long));
    printf("sizeof(long long)=%d\n", sizeof(long long));
    long long in = 0x1122334455667788;
    long long out = 0;
    ReverseEndian((char*)&in, 8, (char*)&out);
    printf("out=%llx\n", out);

    Contact c1 = {
        "hzw",
        0x1122334455667788
    };

    int totalLen = PackContact(&c1, 0, 0);
    assert(totalLen == 8 + 1 + 3);

    char *buf = (char*)HZW_MALLOC(totalLen);
    assert(totalLen == PackContact(&c1, buf, totalLen));
    
    Contact c2;
    assert(totalLen == UnpackContact(&c2, buf, totalLen));
    assert(c1.Phone == c2.Phone);
    assert(strcmp(c1.Name, c2.Name) == 0);
    HzwFree(c2.Name);
    HzwFree(buf);

    Contact cs1[3] = {
        {
            "hzw",
            0x1122334455667788
        },
        {
            "golang",
            0x2233445566778899
        },
        {
            "clang",
            0x3344556677889900
        },
    };

    totalLen = PackContacts(cs1, 3, 0, 0);
    assert(totalLen == 12 + 15 + 14);

    buf = (char*)HZW_MALLOC(totalLen);
    assert(totalLen == PackContacts(cs1, 3, buf, totalLen));

    Contact *cs2;
    int cs2Len;
    assert(totalLen == UnpackContacts(&cs2, &cs2Len, buf, totalLen));
    assert(cs2Len == 3);
    for (int i = 0; i < 3; i++) {
        assert(cs1[i].Phone == cs2[i].Phone);
        assert(strcmp(cs1[i].Name, cs2[i].Name) == 0);
        HzwFree(cs2[i].Name);
    }
    HzwFree(cs2);
    HzwFree(buf);
}