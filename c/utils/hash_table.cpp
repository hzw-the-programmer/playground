#include <stdlib.h> // malloc, calloc, free
#include <stdio.h> // NULL
#include <string.h> // strdup, strlen, strcmp
#include <math.h> // pow

#include "hash_table.h"

static ht_item* ht_new_item(const char *k, const char *v) {
	ht_item *i = (ht_item*)malloc(sizeof(ht_item));
	i->key = strdup(k);
	i->value = strdup(v);
	return i;
}

ht_hash_table* ht_new() {
	ht_hash_table *ht = (ht_hash_table*)malloc(sizeof(ht_hash_table));
	ht->size = 53;
	ht->count = 0;
	ht->items = (ht_item**)calloc(ht->size, sizeof(ht_item*));
	return ht;
}

static void ht_del_item(ht_item *i) {
	free(i->key);
	free(i->value);
	free(i);
}

void ht_del_hash_table(ht_hash_table *ht) {
	for (int i = 0; i < ht->size; i++) {
		ht_item *item = ht->items[i];
		if (item != NULL) {
			ht_del_item(item);
		}
	}
	free(ht->items);
	free(ht);
}

static int ht_hash(const char *s, const int a, const int m) {
	long hash = 0;
	const int len_s = strlen(s);
	for (int i = 0; i < len_s; i++) {
		hash += s[i] * (long)pow((float)a, len_s - (i+1));
	}
	hash %= m;
	return (int)hash;
}

#define HT_PRIME_1 151
#define HT_PRIME_2 163

static int ht_get_hash(const char *s, const int num_buckets, const int attempt) {
	const int hash_a = ht_hash(s, HT_PRIME_1, num_buckets);
	const int hash_b = ht_hash(s, HT_PRIME_2, num_buckets);
	return (hash_a + attempt * (hash_b + 1)) % num_buckets;
}

void ht_insert(ht_hash_table *ht, const char *k, const char *v) {
	ht_item *item = ht_new_item(k, v);
	int index = ht_get_hash(item->key, ht->size, 0);
	ht_item *cur_item = ht->items[index];
	int i = 1;
	while (cur_item) {
		index = ht_get_hash(item->key, ht->size, i);
		cur_item = ht->items[index];
		i++;
	}
	ht->items[index] = item;
	ht->count++;
}

char* ht_search(ht_hash_table *ht, const char *key) {
	int index = ht_get_hash(key, ht->size, 0);
	ht_item *item = ht->items[index];
	int i = 1;
	while (item) {
		if (strcmp(item->key, key) == 0) {
			return item->value;
		}
		index = ht_get_hash(key, ht->size, i);
		item = ht->items[index];
		i++;
	}
	return NULL;
}
