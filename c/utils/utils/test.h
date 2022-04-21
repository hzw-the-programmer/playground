#if !defined(__TEST_H__)
#define __TEST_H__
typedef void TestSetUp();
typedef void TestFunc();
typedef void TestTearDown();

typedef struct {
    TestSetUp *setup;
    TestFunc *func;
    TestTearDown *teardown;
} TestItem;

void debug_buf_clear();
void debug_buf_append(const char *buf);
void debug_buf_assert(const char *buf);
void test_run(TestItem *items, int len);
#endif
