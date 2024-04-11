/* Com base na descrição da struct tm, 
 crie um programa que mostre quantos dias se passaram no ano atual */

#include <stdio.h>
#include <time.h>

int get_days_numbers(int month);
int sum_months_by_day(int current_month);

int main() {

  time_t t = time(NULL);

  struct tm tm = *localtime(&t);
  
  int current_month = tm.tm_mon + 1;
  
  printf("In the year %d have passed %02d days so far", tm.tm_year + 1900, sum_months_by_day(current_month));
  
}

int sum_months_by_day(int current_month){
  
  int days = 0;

  for (int month = 0; month <= current_month; month++) {

    days += get_days_numbers(month);
  }

  return days;

}

int get_days_numbers(int month) {

  if ( month == 02) {
    return 29;
  } else if ( month == 4 || month == 6 || month == 9 || month == 11 ) {
    return 30;
  } else {
    return 31;
  }
}
