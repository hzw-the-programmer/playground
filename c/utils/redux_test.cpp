#include <stdio.h>
#include "redux.h"

void counter_reducer(State *state, Action *action) {
	switch (action->type) {
		case COUNTER_INCREASE:
			state->counter++;
			break;
		case COUNTER_DECREASE:
			state->counter--;
			break;
		case COUNTER_INCREASE_BY:
			state->counter += (int)action->payload;
			break;
		case COUNTER_DECREASE_BY:
			state->counter -= (int)action->payload;
			break;
	}
}

void counter_subscriber(State *state) {
	printf("counter=%d\n", state->counter);
}

void test_redux(void) {
	Action action;

	action.type = COUNTER_INCREASE;
	dispatch(&action);

	action.type = COUNTER_INCREASE_BY;
	action.payload = (void*)4;
	dispatch(&action);

	action.type = COUNTER_DECREASE;
	dispatch(&action);

	action.type = COUNTER_DECREASE_BY;
	action.payload = (void*)10;
	dispatch(&action);
}
