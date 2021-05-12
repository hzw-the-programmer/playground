#include <stdlib.h>
#include <stdio.h>

typedef struct node *link;
struct node {
    int item;
    link next;
};

void print(link l) {
    printf("%d ", l->item);
}

void traverse(link l, void (*visit)(link)) {
    for (link t = l; t; t = t->next) {
        visit(t);
    }
}

int main(int argc, char *argv[]) {
    struct node heada, headb;
    link a, b, t, x, y;

    a = t = &heada;

    int N = atoi(argv[1]);
    for (int i = 0; i < N; i++) {
        t = t->next = malloc(sizeof(*t));
        t->item = random() % 1000;
        t->next = NULL;
    }

    traverse(a->next, print);
    printf("\n");

    b = &headb;
    b->next = NULL;

    for (x = a->next; x; x = t) {
        t = x->next;
        for (y = b; y->next; y = y->next) {
            if (y->next->item > x->item) break;
        }
        x->next = y->next;
        y->next = x;
    }

    traverse(b->next, print);
    printf("\n");
}
