#if !defined(__UTILS_H__)
#define __UTILS_H__
#define ARRAY_SIZE(arr) ((sizeof(arr)) / (sizeof(arr[0])))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
int is_space(char c);
#endif
