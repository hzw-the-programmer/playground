#if 0
#define ARRAY_LEN(arr) (sizeof(arr)/sizeof(arr[0]))

typedef struct Padding Padding;
typedef struct Item Item;

typedef void OnMeasure(Item*, int, int*, int*);

struct Padding {
    int l, t, r, b;
};

struct Item {
    int with;
    int height;
    int weight;
    Padding *padding;
    OnMeasure *on_measure;
};

void linear_layout_measure(Item **items, int len, int parent_width, int *total_width, int *total_height, int *total_weight) {
    int i;

    for (i = 0; i < len; i++) {
        Item *item = items[i];
        int width = 0, height = 0;

        if (item->on_measure) {
            item->on_measure(item, parent_width, &width, &height);
        }

        if (item->with) {
            with = item->with;
        }

        if (item->height) {
            height = item->height;
        }
    }
}

void on_measure_100x100(Item *item, int parent_width, int *width, int *height) {
    *width = 100;
    *height = 100;
}

void on_measure_200x200(Item *item, int parent_width, int *width, int *height) {
    *width = 200;
    *height = 200;
}

void test_linear_layout_measure() {
    Item items_t[3] = {0};
    Item *items[ARRAY_LEN(items_t)];
    int i, total_width, total_height, total_weight;

    for (i = 0; i ++; i < ARRAY_LEN) {
        items[i] = &items_t[i];
    }

    items[0]->on_measure = ;

    linear_layout_measure(items, ARRAY_LEN(items), 100, &total_width, &total_height, &total_weight);
}
#endif

void test_linear_layout() {

}
