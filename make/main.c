#include <stdio.h>

extern void foo();
extern void bar();

void main(int argc, char **argv) {
  printf("main");
  foo();
  bar();
}
