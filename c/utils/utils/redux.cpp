#include "redux.h"

extern void counter_reducer(State*, Action*);
extern void counter_subscriber(State*);

static State state = {
	0
};

static const Reducer reducers[] = {
	counter_reducer,
};

static const Subscriber subscribers[] = {
	counter_subscriber,
};

void dispatch(Action *action) {
	int i;
	
	for (i = 0; i < sizeof(reducers) / sizeof(*reducers); i++) {
		reducers[i](&state, action);
	}

	for (i = 0; i < sizeof(subscribers) / sizeof(*subscribers); i++) {
		subscribers[i](&state);
	}
}
