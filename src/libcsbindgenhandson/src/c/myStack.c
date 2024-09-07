#include "myStack.h"
#include <stdlib.h>

MyStack *myStack_create(int maxLength)
{
    MyStack *ret = malloc(sizeof(MyStack));
    if (ret == NULL)
    {
        return NULL;
    }
    int *data = malloc(sizeof(int) * maxLength);
    if (data == NULL)
    {
        free(ret);
        return NULL;
    }
    ret->index = -1;
    ret->data = data;
    return ret;
}

int myStack_pop(MyStack *myStack){
    return myStack->data[myStack->index--];
}

void myStack_push(MyStack *myStack, int val){
    myStack->data[++myStack->index]=val;
}

void myStack_delete(MyStack *myStack){
    free(myStack->data);
    free(myStack);
}
