#define HELLO "hello"
#define HELLO_WORLD HELLO" world"

#define TO_WS(s) L"s"

void test_preprocessor() {
    char *s = HELLO_WORLD;
    wchar_t *ws = TO_WS(HELLO_WORLD);
    ws = 0;
}
