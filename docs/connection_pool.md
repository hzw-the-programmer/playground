```
typedef struct {
} Conn;

typedef struct {
} ConnPool;

ConnPool *pool = NULL;

void ConnPoolInit() {
}

Conn* ConnPoolGet() {
}

void ConnPoolPut(Conn *conn) {
}

typedef struct {
    int cancelled;
    int (*cb)(HttpResponse*);
} HttpRequest;

typedef struct {
} HttpResponse;

HttpRequest* HttpRequestCreate(int method, char *url) {
}

HttpResponse* HttpResponseCreate(void (*cb)(HttpResponse*)) {
    HttpResponse *response = malloc(sizeof(HttpResponse));

    response->

    return response;
}

HttpResponse* MakeHttpRequest(HttpRequest *request) {
    Conn *conn = NULL;
    HttpResponse *response = NULL;

    conn = ConnPoolGet();
    if (!conn) return NULL;


}

int main() {
    HttpRequest *request = NULL;
    HttpResponse *response = NULL;

    ConnPoolInit();

    request = HttpRequestCreate(GET, "www.google.com");
    response = MakeHttpRequest(request);
}
```
