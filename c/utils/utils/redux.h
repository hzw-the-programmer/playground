typedef struct {
	int counter;
} State;

typedef struct {
	int type;
	void *payload;
} Action;

typedef enum {
	COUNTER_INCREASE,
	COUNTER_DECREASE,
	COUNTER_INCREASE_BY,
	COUNTER_DECREASE_BY,
};

typedef void (*Reducer)(State*, Action*);
typedef void (*Subscriber)(State*);

extern void dispatch(Action *action);
