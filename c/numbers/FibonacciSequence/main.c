// fibonacci sequence

#include <stdio.h>



int main(){

  int N1 = 0;
  int N2 = 1;
  int FIBO = 0; 

  printf("%d, ", N1);
  printf("%d, ", N2);


  while (FIBO < 100000) {
    FIBO = N1 + N2;
    printf("%d, ", FIBO);
    N1 = N2;
    N2 = FIBO;

  }



  return 0;
}
