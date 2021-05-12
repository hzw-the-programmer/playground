#define TEST

#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

typedef int elem_t;

typedef struct stack {
    size_t size;
    size_t top;
    elem_t *items;
} stack_t;

stack_t* createStack(size_t size) {
    stack_t *stack;

    stack = malloc(sizeof(stack_t));
    
    if (stack == NULL) return NULL;

    stack->size = size;
    stack->top = 0;
    
    stack->items = malloc(size * sizeof(elem_t));
    if (stack->items == NULL) {
        free(stack);
        return NULL;
    }

    return stack;
}

void destroyStack(stack_t *stack) {
    assert(stack != NULL);

    free(stack->items);
    free(stack);
}

int isEmpty(stack_t *stack) {
    assert(stack != NULL);

    return stack->top == 0 ? 1 : 0;
}

int isFull(stack_t *stack) {
    assert(stack != NULL);

    return stack->top == stack->size ? 1 : 0;
}

void push(stack_t *stack, elem_t e) {
    assert(!isFull(stack));

    stack->items[stack->top++] = e;
}

elem_t pop(stack_t *stack) {
    assert(!isEmpty(stack));

    return stack->items[--stack->top];
}

#ifdef TEST

int main(int argc, char *argv[]) {
    size_t size = 10;
    stack_t *stack = createStack(size);
    if (stack == NULL) {
        fprintf(stderr, "createStack failed.\n");
    }

    for (int i = 0; i < size; i++) {
        push(stack, i);
    }

    for (int i = 0; i < size; i++) {
        printf("%d ", pop(stack));
    }
    printf("\n");
}

#endif
