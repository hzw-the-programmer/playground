/*
typedef struct {
	int fd, size, r, w;
	char *buf;
} IOBuffer;

IOBuffer* IOBuffer_new(int fd, int size) {
	IOBuffer *iobuf = malloc(sizeof(IOBuffer));
	if (!iobuf) return NULL;

	iobuf->fd = fd;
	iobuf->size = size;
	iobuf->r = 0;
	iobuf->w = 0;
	
	iobuf->buf = malloc(size);
	if (!iobuf->buf) {
		free(iobuf);
		return NULL;
	}

	return buf;
}

char IOBuffer_read
*/
