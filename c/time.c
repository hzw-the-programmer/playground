#include <time.h>
#include <assert.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    time_t ts;
    struct tm dt, *dtp;

    ts = 0;
    dtp = gmtime_r(&ts, &dt);
    assert(dtp == &dt);
    printf("%d-%d-%d\n", dt.tm_year, dt.tm_mon, dt.tm_mday);
    printf("%d:%d:%d\n", dt.tm_hour, dt.tm_min, dt.tm_sec);
    printf("%d %d %d\n", dt.tm_wday, dt.tm_yday, dt.tm_isdst);

    ts = 0;
    dtp = localtime_r(&ts, &dt);
    assert(dtp == &dt);
    printf("%d-%d-%d\n", dt.tm_year, dt.tm_mon, dt.tm_mday);
    printf("%d:%d:%d\n", dt.tm_hour, dt.tm_min, dt.tm_sec);
    printf("%d %d %d\n", dt.tm_wday, dt.tm_yday, dt.tm_isdst);
    assert(0 == mktime(&dt));

    ts = -28800;
    dtp = gmtime_r(&ts, &dt);
    assert(dtp == &dt);
    printf("%d-%d-%d\n", dt.tm_year, dt.tm_mon, dt.tm_mday);
    printf("%d:%d:%d\n", dt.tm_hour, dt.tm_min, dt.tm_sec);
    printf("%d %d %d\n", dt.tm_wday, dt.tm_yday, dt.tm_isdst);

    ts = -28800;
    dtp = localtime_r(&ts, &dt);
    assert(dtp == &dt);
    printf("%d-%d-%d\n", dt.tm_year, dt.tm_mon, dt.tm_mday);
    printf("%d:%d:%d\n", dt.tm_hour, dt.tm_min, dt.tm_sec);
    printf("%d %d %d\n", dt.tm_wday, dt.tm_yday, dt.tm_isdst);
    assert(-28800 == mktime(&dt));
}
