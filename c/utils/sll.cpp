#include "memory.h"

#define NULL 0

typedef struct node {
	int data;
	bool del;
	struct node *next;
} node;

static void add(node **head, int data) {
	node *n = (node*)HZW_MALLOC(sizeof(node));
	
	if (!n) return;
	
	n->data = data;
	n->del = false;
	n->next = NULL;

	while(*head) {
		head = &(*head)->next;
	}

	*head = n;
}

static node* get(node *head, int data) {
	while (head) {
		if (head->data == data) {
			return head;
		}
		head = head->next;
	}
}

static void rmdel(node **head) {
	while (*head) {
		if ((*head)->del) {
			node *t = *head;
			*head = (*head)->next;
			HzwFree(t);
		} else {
			head = &(*head)->next;
		}
	}
}

void testSll() {
	node *head = NULL;

	for (int i = 0; i < 10; i++) {
		add(&head, i);
	}

	for (int i = 0; i < 10; i+=2) {
		node *n = get(head, i);
		n->del = true;
	}

	rmdel(&head);
}
