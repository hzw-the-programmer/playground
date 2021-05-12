typedef struct circular_que {
    size_t head;
    size_t tail;
    size_t nelem;
    size_t elsize;
    void *data;
} circular_que_t;

extern circular_que_t* create_circular_que(size_t nelem, size_t elsize);
extern void destroy_circular_que(circular_que_t *que);
extern int circular_que_is_empty(circular_que_t *que);
extern int circular_que_is_full(circular_que_t *que);
extern int circular_que_push(circular_que_t *que, void *elem);
extern int circular_que_pop(circular_que_t *que, void *elem);
