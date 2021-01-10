#include <stdio.h>

int main(void) {

  char c = getchar();

  if(c) {
    printf("you entry char is:\n");
    putchar(c);
  }

  return 0;
}