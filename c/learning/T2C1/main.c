// 3. Escreva um programa em C  que leia um valor inicial A
// e imprima a sequência de valores do cálculo de fatorial de A(A!)
// e o seu resultado. Ex: 5! = 5 X 4 X 3 X 2 X 1 = 120.

#include <stdio.h>

int get_factorial(int n) {

  int result = n;

  for ( int i = result - 1 ; i > 0; i-- ){
     
    result *= i;
  }

  return result;

}

int main() {

  int value;

  scanf_s("%d", &value);
  
  int value_factorial = get_factorial(value);

  printf("%d", value_factorial);
 
  return 0;
}
