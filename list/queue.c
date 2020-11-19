#include <stdio.h>
#include <stdlib.h>

#define SIZE 256

typedef struct {
    int* data;
    int top;
    int tail;
} queue;

int init(queue* q) {
    q->data = (int*)malloc(SIZE * sizeof(int));
    if (q->data == NULL) {
        return -1;
    }
    for (int i = 0; i < SIZE; i++) {
        q->data[i] = 0;
    }
    q->top = 0;
    q->tail = 0;
    return 0;
}

int delete(queue* q) {
    free(q->data);
    return 0;
}

int enqueue(queue* q, int val) {
    if (q->tail >= SIZE) {
        printf("queue is full\n");
        return -1;
    }
    q->data[(q->tail)++] = val;
    return 0;
}

int dequeue(queue* q) {
    if (q->top < 0) {
        printf("queue is empty\n");
        return -1;
    }
    return q->data[(q->top)++];
}

void print_queue(queue* q) {
    printf("queue : ");
    for (int i = q->top; i < q->tail; i++) {
        printf("%d ", q->data[i]);
    }
    printf("\n");
}

int main(int argc, char *argv[]) {
    queue q;
    init(&q);
    for (int i = 0; i < 100; i++) {
        printf("enqueue\n");
        enqueue(&q, i);
    }
    print_queue(&q);
    for (int i = 0; i < 50; i++) {
        dequeue(&q);
    }
    print_queue(&q);
    return 0;
}
