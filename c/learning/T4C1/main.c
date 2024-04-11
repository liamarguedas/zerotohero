
#include <stdio.h>

int main() {

  int input;

  scanf("%d", &input);

  int n1 = 0;
  int n2 = 1;
  int fibo = 0;

  for (int loop = 0; loop < input; loop++) {

    fibo = n1 + n2;

    printf("%d\n", fibo);
    

    n1 = n2;
    n2 = fibo;
  }

  return 0;
}
