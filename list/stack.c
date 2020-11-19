#include <stdio.h>
#include <stdlib.h>
#define SIZE 256

typedef 
struct {
    int* data;
    int size;
} stack;


int init(stack* s) {
    s->data = (int *)malloc(SIZE * sizeof(int));
    if (s->data == NULL) {
        return -1;
    }
    for (int i = 0; i < SIZE; i++) {
        s->data[i] = 0;
    }
    s->size = -1;
    return 0;
}

int delete(stack* s) {
    free(s->data);
    return 0;
}
int push(stack* s, int val) {
    if (s->size >= SIZE) {
        printf("stack overflow\n");
        return -1;
    }
    
    s->data[++(s->size)] = val;
    return 0;
}

int pop(stack* s) {
    if (s->size == 0) {
        printf("stack is empty\n");
        return -1;
    }
    s->size--;
    int val = s->data[s->size];
    s->data[s->size+1] = 0;
    return val;
}

void print_stack(stack* s) {
    printf("stack : ");
    for (int i = s->size;  0 <= i; i--) {
        printf("%d ", s->data[i]);
    }
    printf("\n");
}

int main(int argc, char *argv[]) {
    stack s;
    init(&s);
    for (int i = 0; i < 100; i++) {
        push(&s, i);
    }
    print_stack(&s);
    for (int i = 0; i < 50; i++) {
        pop(&s);
    }
    print_stack(&s);
    return 0;
}
