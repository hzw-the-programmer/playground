#include <stdlib.h>
#include <stdio.h>

typedef struct node *link;
struct node {
    int item;
    link next;
};

void printnode(link x) {
    printf("%d ", x->item);
}

void traverse(link x, void (*visit)(link)) {
    while (x) {
        visit(x);
        x = x->next;
    }
}

link reverse(link c) {
    link p = NULL, n;

    while (c) {
        n = c->next;
        c->next = p;
        p = c;
        c = n;
    }

    return p;
}

int main(int argc, char *argv[]) {
    int i, N = atoi(argv[1]);

    link t = malloc(sizeof(*t)), x = t;
    x->item = 1; x->next = NULL;

    for (i = 2; i <= N; i++) {
        x = x->next = malloc(sizeof(*x));
        x->item = i; x->next = NULL;
    }

    traverse(t, printnode);
    printf("\n");

    traverse(reverse(t), printnode);
    printf("\n");
}
