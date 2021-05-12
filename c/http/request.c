typedef struct {
    char *host;
    int port;
    int method;
    char *path;
    void (*headers)(char *buf);
    void (*body)(char *buf);
} request;
