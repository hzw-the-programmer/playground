#if !defined(__BUF_H__)
#define __BUF_H__

typedef struct {
    char *data;
    int cap;
    int len;
} buf_t;

int buf_reserve(buf_t *buf, int len);
int buf_write(buf_t *buf, const char *data, int len);
int buf_drain(buf_t *buf, int len);
int buf_append_header(buf_t *buf, const char *key, const char *value);
int buf_delete_header(buf_t *buf, const char *key);

#endif // __BUF_H__
