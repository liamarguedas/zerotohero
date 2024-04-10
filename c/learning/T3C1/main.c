
# include <stdio.h>
# include <time.h>

int main() {
  time_t seg;
  seg = time(NULL);

  printf("Hours since 1970: %lld", seg/3600);
}
