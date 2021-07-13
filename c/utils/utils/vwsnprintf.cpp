#include <assert.h>
#include <cwchar>
#include <stdarg.h>

static void out(size_t idx, wchar_t wc, wchar_t *ws, size_t len) {
    if (idx < len) {
        ws[idx] = wc;
    }
}

static bool is_digit(wchar_t wc) {
    return wc >= L'0' && wc <= L'9';
}

static unsigned int atoi(const wchar_t **ws) {
    unsigned int i = 0;
    while (is_digit(**ws)) {
        i = i * 10 + *((*ws)++) - L'0';
    }
    return i;
}

static size_t vwsnprintwf(wchar_t *ws, size_t len, const wchar_t *wf, va_list args) {
    size_t idx = 0;
    int width;

    if (ws == NULL) {
        len = 0;
    }

    while (*wf) {
        if (*wf != L'%') {
            out(idx++, *wf++, ws, len);
            continue;
        }

        wf++;

        if (*wf == L'0') {
            wf++;
        }

        width = 0;
        if (is_digit(*wf)) {
            width = atoi(&wf);
        }

        switch (*wf) {
            case L'c': {
                char c = (char)va_arg(args, int);
                out(idx++, c, ws, len);
                break;
            }
            case L's': {
                char *s = va_arg(args, char*);
                while (*s) {
                    out(idx++, *s++, ws, len);
                }
                break;
            }
            case L'w': {
                wchar_t *tws = va_arg(args, wchar_t*);
                while (*tws) {
                    out(idx++, *tws++, ws, len);
                }
                break;
            }
            case L'd': {
                int i = va_arg(args, int);
                bool neg = i < 0;
                char buf[64] = {0};
                size_t l = 0;
                int j;
                
                i = neg ? -i : i;
                
                do {
                    buf[l++] = i % 10 + '0';
                    i /= 10;
                } while (i != 0);
                
                if (neg) {
                    out(idx++, L'-', ws, len);
                    width -= 1;
                }

                for (j = l; j < width; j++) {
                    out(idx++, L'0', ws, len);
                }

                while (l > 0) {
                    out(idx++, buf[--l], ws, len);
                }
                break;
            }
        }

        wf++;
    }

    if (idx < len) {
        ws[idx] = 0;
    } else if (len > 0) {
        ws[len - 1] = 0;
    }

    return idx;
}

static size_t wsprintwf(wchar_t *ws, const wchar_t *wf, ...) {
    va_list args;
    size_t idx;

    va_start(args, wf);
    idx = vwsnprintwf(ws, (size_t)-1, wf, args);
    va_end(args);

    return idx;
}

static void test_basic() {
    assert(sizeof(wchar_t) == 2);
}

static void test_wsprintf() {
    wchar_t dst[256] = {0};
    wchar_t *name = L"hzw";
    int age = 35;

    assert(wsprintwf(NULL, L"hello world!") == 12);

    assert(wsprintwf(dst, L"hello world!") == 12);
    assert(wcscmp(dst, L"hello world!") == 0);

    assert(wsprintwf(dst, L"hello %w!", name) == 10);
    assert(wcscmp(dst, L"hello hzw!") == 0);

    assert(wsprintwf(dst, L"hello %s!", "hzw") == 10);
    assert(wcscmp(dst, L"hello hzw!") == 0);

    assert(wsprintwf(dst, L"hello %w %s!", name, "hzw") == 14);
    assert(wcscmp(dst, L"hello hzw hzw!") == 0);

     assert(wsprintwf(dst, L"h%cllo %w %s!", 'e', name, "hzw") == 14);
    assert(wcscmp(dst, L"hello hzw hzw!") == 0);

    assert(wsprintwf(dst, L"%w [%d]", name, age) == 8);
    assert(wcscmp(dst, L"hzw [35]") == 0);

    assert(wsprintwf(dst, L"%w %w", name, name) == 7);
    assert(wcscmp(dst, L"hzw hzw") == 0);

    assert(wsprintwf(dst, L"s%w", name) == 4);
    assert(wcscmp(dst, L"shzw") == 0);

    assert(wsprintwf(dst, L"%w", name) == 3);
    assert(wcscmp(dst, L"hzw") == 0);

    assert(wsprintwf(dst, L"\\%s.%s", "hzw", "txt") == 8);
    assert(wcscmp(dst, L"\\hzw.txt") == 0);

    assert(wsprintwf(dst, L"%d%w", 35, L"txt") == 5);
    assert(wcscmp(dst, L"35txt") == 0);

    assert(wsprintwf(dst, L"%c:\\%s", 'h', "txt") == 6);
    assert(wcscmp(dst, L"h:\\txt") == 0);

    assert(wsprintwf(dst, L"%02d:%02d %w", 1, 2, name) == 9);
    assert(wcscmp(dst, L"01:02 hzw") == 0);

    assert(wsprintwf(dst, L"please retry in %ds.", 10000) == 23);
    assert(wcscmp(dst, L"please retry in 10000s.") == 0);

    assert(wsprintwf(dst, L"%d %w", -10000, name) == 10);
    assert(wcscmp(dst, L"-10000 hzw") == 0);

    assert(wsprintwf(dst, L"%s %w", "hzw", name) == 7);
    assert(wcscmp(dst, L"hzw hzw") == 0);

    assert(wsprintwf(dst, L"+%d (%s)", 123, "hzw") == 10);
    assert(wcscmp(dst, L"+123 (hzw)") == 0);

    assert(wsprintwf(dst, L"%w (%d/%d)", name, -100, 10000) == 16);
    assert(wcscmp(dst, L"hzw (-100/10000)") == 0);

    assert(wsprintwf(dst, L"+%d  (%s)", -1, "hzw") == 10);
    assert(wcscmp(dst, L"+-1  (hzw)") == 0);

    assert(wsprintwf(dst, L"SIM%d: +%d %s", -1, 0, "hzw") == 13);
    assert(wcscmp(dst, L"SIM-1: +0 hzw") == 0);

    assert(wsprintwf(dst, L"00:%02d", 1) == 5);
    assert(wcscmp(dst, L"00:01") == 0);

    assert(wsprintwf(dst, L"00:%02d", 11) == 5);
    assert(wcscmp(dst, L"00:11") == 0);

    assert(wsprintwf(dst, L"%w %02d:%02d", name, 11, 2) == 9);
    assert(wcscmp(dst, L"hzw 11:02") == 0);
}

void test_vwsnprintf() {
    test_basic();
    test_wsprintf();
}
