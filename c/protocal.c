#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <time.h>
#include <assert.h>

#include "buffer.h"

#define TEST

#define VER_LEN 2
#define LEN_LEN 2
#define SN_LEN 8
#define ID_LEN 4
#define DT_LEN 6
#define CMD_LEN 1

enum decode_phase {
    UNPACK_VER,
    UNPACK_LEN,
    CHECK,
    UNPACK_SN,
    UNPACK_ID,
    UNPACK_DT,
    UNPACK_CMD,
    UNPACK_41,
    FINISH
};

typedef struct pkt {
    char ver[VER_LEN + 1];
    int len;
    uint64_t sn;
    uint32_t id;
    struct tm dt;
    uint8_t cmd;
    int valid;
    enum decode_phase decodePhase;
} pkt_t;

int unpack_ver(buffer_t *buf, pkt_t *pkt) {
    uint8_t data[VER_LEN];

    if (getBufferReadLen(buf) < VER_LEN) {
        return -1;
    }

    readFromBuffer(buf, data, VER_LEN);

    memcpy(pkt->ver, data, VER_LEN);
    pkt->ver[VER_LEN] = '\0';

    pkt->decodePhase = UNPACK_LEN;

    return VER_LEN;
}

int unpack_len(buffer_t *buf, pkt_t *pkt) {
    uint8_t data[LEN_LEN];

    if (getBufferReadLen(buf) < LEN_LEN) {
        return -1;
    }

    readFromBuffer(buf, data, LEN_LEN);

    pkt->len = *data < 8 | *(data + 1);

    pkt->decodePhase = CHECK;

    return LEN_LEN;
}

int check(buffer_t *buf, pkt_t *pkt) {
    uint8_t data;
    uint8_t chksum = 0;

    if (getBufferReadLen(buf) < pkt->len) {
        return -1;
    }

    for (int i = 0; i < pkt->len - 1; i++) {
        readFromBuffer(buf, &data, 1);
        chksum ^= data;
    }

    readFromBuffer(buf, &data, 1);
    if (chksum == data) {
        pkt->valid = 1;

        pkt->decodePhase = UNPACK_SN;
        deAdvanceBufferRead(buf, pkt->len);

        return 0;
    } else {
        pkt->valid = 0;

        pkt->decodePhase = FINISH;

        return pkt->len;
    }
}

int unpack_sn(buffer_t *buf, pkt_t *pkt) {
    uint8_t data[SN_LEN];

    if (getBufferReadLen(buf) < SN_LEN) {
        return -1;
    }

    readFromBuffer(buf, data, SN_LEN);

    pkt->sn = *(uint64_t*)data;

    pkt->decodePhase = UNPACK_ID;

    return SN_LEN;
}

int unpack_id(buffer_t *buf, pkt_t *pkt) {
    uint8_t data[ID_LEN];

    if (getBufferReadLen(buf) < ID_LEN) {
        return -1;
    }

    readFromBuffer(buf, data, ID_LEN);

    for (int i = 0; i < ID_LEN; i++) {
        pkt->id <<= 8;
        pkt->id |= *(data + i);
    }

    pkt->decodePhase = UNPACK_DT;

    return ID_LEN;
}

int unpack_dt(buffer_t *buf, pkt_t *pkt) {
    uint8_t data[DT_LEN];

    if (getBufferReadLen(buf) < DT_LEN) {
        return -1;
    }

    readFromBuffer(buf, data, DT_LEN);

    pkt->dt.tm_year = *(data + 0) + 2000 - 1900;
    pkt->dt.tm_mon = *(data + 1) - 1;
    pkt->dt.tm_mday = *(data + 2);
    pkt->dt.tm_hour = *(data + 3);
    pkt->dt.tm_min = *(data + 4);
    pkt->dt.tm_sec = *(data + 5);

    // pkt->dt.tm_wday = 0;
    // pkt->dt.tm_yday = 0;
    // pkt->dt.tm_isdst = 0;

    pkt->decodePhase = UNPACK_CMD;

    return DT_LEN;
}

int unpack_cmd(buffer_t *buf, pkt_t *pkt) {
    uint8_t cmd;

    if (getBufferReadLen(buf) < CMD_LEN) {
        return -1;
    }

    readFromBuffer(buf, &cmd, CMD_LEN);

    pkt->cmd = cmd;

    switch (cmd) {
        case 0x41:
            pkt->decodePhase = UNPACK_41;
            break;
    }

    return CMD_LEN;
}

void decodeBufferToPkt(buffer_t *buf, pkt_t *pkt) {
    while (pkt->decodePhase != FINISH) {
        if (pkt->decodePhase == UNPACK_VER) {
            if (unpack_ver(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == UNPACK_LEN) {
            if (unpack_len(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == CHECK) {
            if (check(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == UNPACK_SN) {
            if (unpack_sn(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == UNPACK_ID) {
            if (unpack_id(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == UNPACK_DT) {
            if (unpack_dt(buf, pkt) == -1) {
                break;
            }
        } else if (pkt->decodePhase == UNPACK_CMD) {
            if (unpack_cmd(buf, pkt) == -1) {
                break;
            }
        } else {
            break;
        }
    }
}

#ifdef TEST

void testUnpack41() {
    uint8_t pktd[] = {
        0x50, 0x33, //ver
        0x00, 0x23, //len
        0x03, 0xF6, 0x61, 0xFE, 0x01, 0x48, 0x00, 0x00, //sn
        0x00, 0x00, 0x04, 0x4B, //id
        0x12, 0x03, 0x08, 0x10, 0x1A, 0x14, //dt
        0x41, //cmd
        0x02, //num
        0x04, //slot
        0x00, //port
        0x09, //type
        0x11, 0xCD, 0x18, 0x45, //data
        0x04, 0x01, 0x09, 0x77, 0xFB, 0x18, 0x45,
        0x79 //chksum
    };

    buffer_t *buf;
    uint8_t tbuf[1024];
    int len;

    buf = createBuffer(1024);

    len = writeToBuffer(buf, pktd, sizeof(pktd));

    pkt_t pkt = {0};
    decodeBufferToPkt(buf, &pkt);

    assert(strcmp("P3", pkt.ver) == 0);
    assert(35 == pkt.len);
    assert(1 == pkt.valid);
    assert(79173400000003 == pkt.sn);
    assert(1099 == pkt.id);
    char dts[19 + 1];
    strftime(dts, sizeof(dts), "%Y-%m-%d %H:%M:%S", &pkt.dt);
    assert(strcmp("2018-03-08 16:26:20", dts) == 0);
    assert(0x41 == pkt.cmd);

    destroyBuffer(buf);
}

int main(int argc, char *argv[]) {
    testUnpack41();
}

#endif

// struct tm {
//     int tm_sec;         /* seconds */
//     int tm_min;         /* minutes */
//     int tm_hour;        /* hours */
//     int tm_mday;        /* day of the month */
//     int tm_mon;         /* month */
//     int tm_year;        /* year */
//     int tm_wday;        /* day of the week */
//     int tm_yday;        /* day in the year */
//     int tm_isdst;       /* daylight saving time */
// };
