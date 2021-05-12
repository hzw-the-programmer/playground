#include <stdio.h>
#include <string.h>
#include <assert.h>
#include <time.h>

#include "utils.h"

#include "hprotocal.h"

#define TEST

#define VER_INDEX 0
#define VER_LEN 2
#define LEN_INDEX 2
#define LEN_LEN 2
#define SN_INDEX 4
#define SN_LEN 8
#define ID_INDEX 12
#define ID_LEN 4
#define DT_INDEX 16
#define DT_LEN 6
#define CMD_INDEX 22
#define CMD_LEN 1

ssize_t get_ver(const udp_pkt_t *pkt, char *ver, size_t len) {
    const uint8_t *p;

    if (pkt->len < VER_INDEX + VER_LEN) {
        return -1;
    }
    if (len < VER_LEN) {
        return -1;
    }

    p = pkt->data + VER_INDEX;

    *ver = *p;
    *(ver + 1) = *(p + 1);

    return VER_LEN;
}

ssize_t get_len(const udp_pkt_t *pkt, size_t *len) {
    const uint8_t *p;

    if (pkt->len < LEN_INDEX + LEN_LEN) {
        return -1;
    }

    *len = 0;
    p = pkt->data + LEN_INDEX;
    for (int i = 0; i < LEN_LEN; i++) {
        *len <<= 8;
        *len |= *p;
        p++;
    }

    return LEN_LEN;
}

ssize_t get_sn(const udp_pkt_t *pkt, uint64_t *sn) {
    if (pkt->len < SN_INDEX + SN_LEN) {
        return -1;
    }

    *sn = *(uint64_t*)(pkt->data + SN_INDEX);

    return SN_LEN;
}

ssize_t get_id(const udp_pkt_t *pkt, uint32_t *id) {
    const uint8_t *p;

    if (pkt->len < ID_INDEX + ID_LEN) {
        return -1;
    }

    *id = 0;
    p = pkt->data + ID_INDEX;
    for (int i = 0; i < ID_LEN; i++) {
        *id <<= 8;
        *id |= *p;
        p++;
    }

    return ID_LEN;
}

ssize_t get_dt(const udp_pkt_t *pkt, struct tm *dt) {
    const uint8_t *p;

    if (pkt->len < DT_INDEX + DT_LEN) {
        return -1;
    }

    p = pkt->data + DT_INDEX;

    dt->tm_year = *p + 2000 - 1900;
    dt->tm_mon = *(p + 1) - 1;
    dt->tm_mday = *(p + 2);
    dt->tm_hour = *(p + 3);
    dt->tm_min = *(p + 4);
    dt->tm_sec = *(p + 5);

    dt->tm_wday = 0;
    dt->tm_yday = 0;
    dt->tm_isdst = 0;

    return DT_LEN;
}

ssize_t get_cmd(const udp_pkt_t *pkt, uint8_t *cmd) {
    if (pkt->len < CMD_INDEX + CMD_LEN) {
        return -1;
    }

    *cmd = *(pkt->data + CMD_INDEX);

    return CMD_LEN;
}

int checksum(const udp_pkt_t *pkt, int *chksum) {
    size_t len;
    const uint8_t *p;

    if (get_len(pkt, &len) == -1) {
        return -1;
    }
    if (pkt->len < SN_INDEX + len) {
        return -1;
    }

    *chksum = 0;
    p = pkt->data + SN_INDEX;
    while (len != 1) {
        *chksum ^= *p;
        p++;
        len--;
    }

    return 0;
}

int valid(const udp_pkt_t *pkt) {
    char ver[VER_LEN];
    size_t len;
    int chksum;

    if (get_ver(pkt, ver, sizeof(ver)) == -1) {
        return 0;
    }
    if ('P' != ver[0] || '3' != ver[1]) {
        return 0;
    }

    if (get_len(pkt, &len) == -1) {
        return 0;
    }

    if (checksum(pkt, &chksum) == -1) {
        return 0;
    }

    if (*(pkt->data + SN_INDEX + len - 1) != chksum) {
        return 0;
    }

    return 1;
}

#ifdef TEST

int main(int argc, char *argv[]) {
    char pktstr[] = "5033001403F661FE0148000000000452120308101E06FE9A";
    udp_pkt_t pkt;
    ssize_t len, plen;
    char ver[VER_LEN + 1];
    uint64_t sn;
    uint32_t id;
    struct tm dt;
    uint8_t cmd;

    assert(strlen(pktstr) + 1 == sizeof(pktstr));
    pkt.len = unhexlify(pktstr, sizeof(pktstr), pkt.data, UDP_PKT_DATA_LEN);

    len = get_ver(&pkt, ver, sizeof(ver));
    assert(VER_LEN == len);
    ver[len] = '\0';
    assert(strcmp("P3", ver) == 0);

    len = get_len(&pkt, &plen);
    assert(LEN_LEN == len);
    assert(20 == plen);

    assert(1 == valid(&pkt));
    len = get_sn(&pkt, &sn);
    assert(SN_LEN == len);
    assert(79173400000003 == sn);

    len = get_id(&pkt, &id);
    assert(ID_LEN == len);
    assert(1106 == id);

    len = get_dt(&pkt, &dt);
    assert(6 == len);
    assert(118 == dt.tm_year);
    assert(2 == dt.tm_mon);
    assert(8 == dt.tm_mday);
    assert(16 == dt.tm_hour);
    assert(30 == dt.tm_min);
    assert(6 == dt.tm_sec);

    len = get_cmd(&pkt, &cmd);
    assert(1 == len);
    assert(0xFE == cmd);
}

#endif

//gcc hprotocal.c utils.c
