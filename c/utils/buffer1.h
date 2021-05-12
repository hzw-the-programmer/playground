typedef struct buffer_s {
    char *data;
    int cap;
    int len;
} buffer_t;

extern void buffer_free(buffer_t *buf);
extern buffer_t* buffer_new(int cap);
extern char* buffer_writable(buffer_t *buf);
extern int buffer_writable_len(buffer_t *buf);
extern char* buffer_readable(buffer_t *buf);
extern int buffer_readable_len(buffer_t *buf);
extern void buffer_inc_len(buffer_t *buf, int len);
extern int buffer_copy(buffer_t *dst, buffer_t *src, int len);
extern int buffer_copy_str(buffer_t *dst, char *str);
extern int buffer_clear(buffer_t *buf, int len);
extern bool buffer_full(buffer_t *buf);
