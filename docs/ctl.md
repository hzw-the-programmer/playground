```
#define CTL_COMMON_FIELDS \
    Rect display;         \
    Rect clip;            \
    void (*draw)(struct Ctl* ctl)

typedef struct Ctl {
    CTL_COMMON_FIELDS;
} Ctl;

typedef struct TxtCtl {
    CTL_COMMON_FIELDS;
    char *txt;
    unsigned int color;
    unsigned int bg_color;
    unsigned int flag;
} TxtCtl;

void DrawCtl(Ctl *ctl) {
}

void InitCtl(
    Ctl *ctl,
    Rect display,
    Rect clip,
    void (*draw)(Ctl*)) {
    ctl->display = display;
    ctl->clip = clip;
    ctl->draw = draw;
}

void DrawTxtCtl(Ctl *ctl) {
    TxtCtl *txtCtl = (TxtCtl*)ctl;

    DrawCtl(ctl);

    DrawText(
        txtCtl->txt,
        &txtCtl->display,
        NULL,
        txtCtl->flag,
        txtCtl->color,
        txtCtl->bg_color);
}

void InitTxtCtl(
    TxtCtl *ctl,
    Rect display,
    Rect clip,
    char *txt,
    unsigned int color,
    unsigned int bg_color,
    unsigned int flag)
{
    InitCtl((Ctl*)ctl, display, clip, DrawTxtCtl);
    ctl->txt = txt;
    ctl->color = color;
    ctl->bg_color = bg_color;
    ctl->flag = flag;
}

TxtCtl* NewTxtCtl(
    Rect display,
    Rect clip,
    char *txt,
    unsigned int color,
    unsigned int bg_color,
    unsigned int flag)
{
    TxtCtl *ctl = NULL;

    ctl = (TxtCtl*)malloc(sizeof(TxtCtl));
    if (!ctl)
    {
        return NULL;
    }

    InitTxtCtl(ctl, display, clip, txt, color, bg_color, flag);

    return ctl;
}
```
