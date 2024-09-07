#ifndef MYSTACK_H_
#define MYSTACK_H_

typedef struct MyStack
{
    int index;
    int *data;
} MyStack;

MyStack *myStack_create(int maxLength);
int myStack_pop(MyStack *myStack);
void myStack_push(MyStack *myStack, int val);
void myStack_delete(MyStack *myStack);

#endif
