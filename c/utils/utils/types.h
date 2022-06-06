#if !defined(__TYPES_H__)
#define __TYPES_H__
typedef enum { false, true } bool;

/* Define NULL pointer value */
#ifndef NULL
#ifdef __cplusplus
#define NULL    0
#else
#define NULL    ((void *)0)
#endif
#endif

#endif
