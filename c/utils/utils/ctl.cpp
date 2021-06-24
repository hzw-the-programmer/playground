#include <stdio.h>

#include "memory.h"

#define NULL 0
typedef unsigned char uint8;

#define CTL_COMMON_FIELDS \
    int x, y, w, h;                                 \
    void (*draw)(Ctl*)

typedef struct Ctl {
    CTL_COMMON_FIELDS;
} Ctl;

void InitCtl(Ctl *ctl, int x, int y, int w, int h, void(*draw)(Ctl*)) {
    ctl->x = x;
    ctl->y = y;
    ctl->w = w;
    ctl->h = h;
    ctl->draw = draw;
}

void DrawCtl(Ctl *ctl) {
    printf("x=%d, y=%d, w=%d, h=%d", ctl->x, ctl->y, ctl->w, ctl->h);
}

typedef struct {
    CTL_COMMON_FIELDS;
    const char *txt;
} TxtCtl;

void DrawTxtCtl(Ctl *ctl) {
    TxtCtl *txtCtl = (TxtCtl*)ctl;

    DrawCtl(ctl);
    printf(", txt=%s\n", txtCtl->txt);
}

void InitTxtCtl(TxtCtl *txtCtl, int x, int y, int w, int h, const char *txt) {
    InitCtl((Ctl*)txtCtl, x, y, w, h, DrawTxtCtl);
    txtCtl->txt = txt;
}

TxtCtl* NewTxtCtl(int x, int y, int w, int h, const char *txt) {
    TxtCtl *ctl = NULL;

    ctl = (TxtCtl*)HZW_MALLOC(sizeof(TxtCtl));
    if (!ctl) return NULL;

    InitTxtCtl(ctl, x, y, w, h, txt);

    return ctl;
}

typedef struct {
    CTL_COMMON_FIELDS;
    const uint8 *img;
} ImgCtl;

void DrawImgCtl(Ctl *ctl) {
    ImgCtl *imgCtl = (ImgCtl*)ctl;

    DrawCtl(ctl);
    printf(", img=[0x%02x, 0x%02x, ...]\n", imgCtl->img[0], imgCtl->img[1]);
}

void InitImgCtl(ImgCtl *ctl, int x, int y, int w, int h, const uint8 *img) {
    InitCtl((Ctl*)ctl, x, y, w, h, DrawImgCtl);
    ctl->img = img;
}

ImgCtl* NewImgCtl(int x, int y, int w, int h, uint8 *img) {
    ImgCtl *ctl = NULL;

    ctl = (ImgCtl*)HZW_MALLOC(sizeof(ImgCtl));
    if (!ctl) return NULL;

    InitImgCtl(ctl, x, y, w, h, img);

    return ctl;
}

void test_ctl() {
    Ctl *ctl = (Ctl*)NewTxtCtl(0, 0, 100, 100, "hello world!");
    ctl->draw(ctl);

    uint8 img[10] = {1, 2, 3};
    ctl = (Ctl*)NewImgCtl(100, 100, 200, 200, img);
    ctl->draw(ctl);
}
