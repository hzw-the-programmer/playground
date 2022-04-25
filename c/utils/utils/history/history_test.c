#include "../test.h"
#include "history.h"

static void a_on_create(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void a_on_resume(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void a_on_pause(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void a_on_destroy(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void a_on_data(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void b_on_create(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void b_on_resume(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void b_on_pause(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void b_on_destroy(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void b_on_data(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void c_on_create(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void c_on_resume(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void c_on_pause(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void c_on_destroy(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void c_on_data(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void d_on_create(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void d_on_resume(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void d_on_pause(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void d_on_destroy(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void d_on_data(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void e_on_create(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void e_on_resume(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void e_on_pause(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void e_on_destroy(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void e_on_data(void *self) {
    debug_buf_append(__FUNCTION__);
}

static void history_test_setup() {
}

static void history_test_teardown() {
    history_clear();
    debug_buf_clear();
}

static void history_test_sa_ba() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy};
    
    history_start(a);
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_sab_bba() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"
                                   
                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "b_on_pause\n"
                                   "b_on_destroy\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy};
    
    history_start(a);
    history_start(b);
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_sabc_bcba() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "c_on_create\n"
                                   "b_on_pause\n"
                                   "c_on_resume\n"
                                   
                                   "c_on_pause\n"
                                   "c_on_destroy\n"
                                   "b_on_resume\n"
                                   
                                   "b_on_pause\n"
                                   "b_on_destroy\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy};
    HistoryItem c = {&c, 0, c_on_create, c_on_resume, c_on_pause, c_on_destroy};
    
    history_start(a);
    history_start(b);
    history_start(c);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_no_history() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"
                                   
                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "a_on_destroy\n"
                                   "b_on_resume\n"
                                   
                                   "b_on_pause\n"
                                   "b_on_destroy";
    
    HistoryItem a = {&a, HISTORY_FLAG_NO_HISTORY, a_on_create, a_on_resume, a_on_pause, a_on_destroy};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy};
    
    history_start(a);
    history_start(b);
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_on_data() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "a_on_data\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy, a_on_data};
    
    history_start(a);
    history_start(a);
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_clear_top_0() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "b_on_pause\n"
                                   "b_on_destroy\n"
                                   
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy};
    
    history_start(a);
    history_start(b);
    history_start(a);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_clear_top_1() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"

                                   "c_on_create\n"
                                   "b_on_pause\n"
                                   "c_on_resume\n"
                                   
                                   "c_on_pause\n"
                                   "c_on_destroy\n"
                                   "b_on_destroy\n"
                                   
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy, a_on_data};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy, b_on_data};
    HistoryItem c = {&c, 0, c_on_create, c_on_resume, c_on_pause, c_on_destroy, c_on_data};
    
    history_start(a);
    history_start(b);
    history_start(c);
    history_start(a);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_replace_before_0() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "c_on_create\n"
                                   "b_on_pause\n"
                                   "b_on_destroy\n"
                                   "c_on_resume\n"
                                   
                                   "b_on_create\n"
                                   "c_on_pause\n"
                                   "c_on_destroy\n"
                                   "b_on_resume\n"
                                   
                                   "b_on_pause\n"
                                   "b_on_destroy\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy, a_on_data};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy, b_on_data};
    HistoryItem c = {&c, 0, c_on_create, c_on_resume, c_on_pause, c_on_destroy, c_on_data};
    
    history_start(a);
    history_start(b);
    history_replace_before(a, c);
    history_replace_before(a, b);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_replace_before_1() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "c_on_create\n"
                                   "b_on_pause\n"
                                   "c_on_resume\n"
                                   
                                   "d_on_create\n"
                                   "c_on_pause\n"
                                   "c_on_destroy\n"
                                   "b_on_destroy\n"
                                   "d_on_resume\n"
                                   
                                   "d_on_pause\n"
                                   "d_on_destroy\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy, a_on_data};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy, b_on_data};
    HistoryItem c = {&c, 0, c_on_create, c_on_resume, c_on_pause, c_on_destroy, c_on_data};
    HistoryItem d = {&d, 0, d_on_create, d_on_resume, d_on_pause, d_on_destroy, d_on_data};
    
    history_start(a);
    history_start(b);
    history_start(c);
    history_replace_before(a, d);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

static void history_test_replace_before_2() {
    const char *want = "a_on_create\n"
                                   "a_on_resume\n"

                                   "b_on_create\n"
                                   "a_on_pause\n"
                                   "b_on_resume\n"
                                   
                                   "c_on_create\n"
                                   "b_on_pause\n"
                                   "c_on_resume\n"

                                   "d_on_create\n"
                                   "c_on_pause\n"
                                   "d_on_resume\n"
                                   
                                   "e_on_create\n"
                                   "d_on_pause\n"
                                   "d_on_destroy\n"
                                   "c_on_destroy\n"
                                   "b_on_destroy\n"
                                   "e_on_resume\n"
                                   
                                   "e_on_pause\n"
                                   "e_on_destroy\n"
                                   "a_on_resume\n"
                                   
                                   "a_on_pause\n"
                                   "a_on_destroy";
    
    HistoryItem a = {&a, 0, a_on_create, a_on_resume, a_on_pause, a_on_destroy, a_on_data};
    HistoryItem b = {&b, 0, b_on_create, b_on_resume, b_on_pause, b_on_destroy, b_on_data};
    HistoryItem c = {&c, 0, c_on_create, c_on_resume, c_on_pause, c_on_destroy, c_on_data};
    HistoryItem d = {&d, 0, d_on_create, d_on_resume, d_on_pause, d_on_destroy, d_on_data};
    HistoryItem e = {&e, 0, e_on_create, e_on_resume, e_on_pause, e_on_destroy, e_on_data};
    
    history_start(a);
    history_start(b);
    history_start(c);
    history_start(d);
    history_replace_before(a, e);
    history_back();
    history_back();
    history_back();
    
    debug_buf_assert(want);
}

void test_history() {
    TestItem items[] = {
        {NULL, history_test_sa_ba, history_test_teardown},
        {NULL, history_test_sab_bba, history_test_teardown},
        {NULL, history_test_sabc_bcba, history_test_teardown},
        {NULL, history_test_no_history, history_test_teardown},
        {NULL, history_test_on_data, history_test_teardown},
        {NULL, history_test_clear_top_0, history_test_teardown},
        {NULL, history_test_clear_top_1, history_test_teardown},
        {NULL, history_test_replace_before_0, history_test_teardown},
        {NULL, history_test_replace_before_1, history_test_teardown},
        {NULL, history_test_replace_before_2, history_test_teardown},
    };

    test_run(items, ARRAY_SIZE(items));
}
