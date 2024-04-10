/*
Desenvolva um programa em C que leia a altura de pessoas,
 cujo número de pessoas é dado pelo usuário. Este programa deverá verificar e mostrar:

a. A menor altura do grupo;

b. A maior altura do grupo.*/

#include <stdio.h> 
#include <string.h>


int get_max(int *array, int n){
 
  int max_value = array[0];  
    
  for (int i = 1; i < n; i++) {
     
    if (array[i] > max_value) {
    
        max_value = array[i];  
          
      }        
    }       
  return max_value;  
  }  

int get_min(int *array, int n){
  
  int min_value = array[0];  
     
  for (int i = 1; i < n; i++) {
      
    if (array[i] < min_value) {
     
        min_value = array[i];  
         
      }        
    }       
  return min_value;  
    
    
  }  
 
 void create_and_validate_array(int n){
 
  int array[n];
  for (int i = 0; i < n; i++ ) {
    array[i] = 0; 
  }


  for (int i = 0; i < n; i++) {
  
    printf("What is the height of the %d person\n", i + 1);

    int current_person_height;

    scanf_s("%d", &current_person_height);

    array[i] = current_person_height;  
  }

  int min = get_min(array, n);  

  int max = get_max(array, n);  
  
  printf("The max is %d and the min is %d", max, min);
  
}


int main() { 
   
  int n_people; 
 
  scanf_s("%d", &n_people);
 
  create_and_validate_array(n_people);
  return 0;
}
