#include <assert.h>

#include "memory.h"

typedef struct Node {
    int data;
    struct Node *next;
} Node;

int push(Node **stack, int data) {
    Node *node = (Node*)HZW_MALLOC(sizeof(Node));
    if (!node) return 0;
    node->data = data;
    node->next = *stack;
    *stack = node;
    return 1;
}

int pop(Node **stack, int *data) {
    Node *node = *stack;
    if (!node) return 0;
    *stack = node->next;
    *data = node->data;
    HzwFree(node);
    return 1;
}

void test_stack() {
    Node *stack = 0;
    int data = 0;
    push(&stack, 1);
    push(&stack, 2);
    assert(pop(&stack, &data));
    assert(data == 2);
    assert(pop(&stack, &data));
    assert(data == 1);

    push(&stack, 1);
    push(&stack, 2);
    assert(pop(&stack, &data));
    assert(data == 2);
    assert(pop(&stack, &data));
    assert(data == 1);
}
