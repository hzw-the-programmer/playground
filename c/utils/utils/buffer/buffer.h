typedef struct {
    char *data;
    int cap;
    int len;
} h_buf;

int h_buf_reserve(h_buf *buf, int len);
int h_buf_write(h_buf *buf, char *data, int len);
int h_buf_drain(h_buf *buf, int len);
