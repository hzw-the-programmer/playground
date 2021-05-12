#include <stdio.h>

//gcc sherlock_holmes.c
//cat sherlock_holmes.input | ./a.out
//SUN 02:02

int main() {
    char s1[60], s2[60], s3[60], s4[60];

    scanf("%s", s1);
    scanf("%s", s2);
    scanf("%s", s3);
    scanf("%s", s4);

    char *DAY[7];
    DAY[0] = "MON";
    DAY[1] = "TUE";
    DAY[2] = "WED";
    DAY[3] = "THU";
    DAY[4] = "FRI";
    DAY[5] = "SAT";
    DAY[6] = "SUN";

    int day = -1, hour = -1, minute = -1;
    for (int i = 0; i < 60; i++) {
        if (s1[i] == s2[i]) {
            if (day == -1 && s1[i] >= 'A' && s1[i] <= 'G') {
                day = s1[i] - 'A';
            } else if (day != -1 && hour == -1) {
                if (s1[i] >= '0' && s1[i] <= '9') {
                    hour = s1[i] - '0';
                } else if (s1[i] >= 'A' && s1[i] <= 'N') {
                    hour = 10 + s1[i] - 'A';
                }
            }
        }
        if (((s3[i] >= 'a' && s3[i] <= 'z') || (s3[i] >= 'A' && s3[i] <= 'Z'))
            && s3[i] == s4[i] && minute == -1
        ) {
            minute = i;
        }
    }
    printf("%s %02d:%02d\n", DAY[day], hour, minute);
}