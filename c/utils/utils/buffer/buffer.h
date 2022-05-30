typedef struct {
    char *data;
    int cap;
    int len;
} h_buf;

int h_buf_reserve(h_buf *buf, int len);
int h_buf_write(h_buf *buf, const char *data, int len);
int h_buf_drain(h_buf *buf, int len);
int h_buf_append_header(h_buf *buf, const char *key, const char *value);
int h_buf_delete_header(h_buf *buf, const char *key);
