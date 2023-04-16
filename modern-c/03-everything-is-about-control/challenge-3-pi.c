#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

long digits(size_t n, double number) {
  return (long) (number * pow(10, n));
}

long digits_of_pi(size_t n) {
  double last_two_results[] = { 3.0, 3.0 };
  double result = 3;
  int sign = 1;
  long next_factor = 2;
  
  while (true) {
    result = result + ((sign * 4.0) / (next_factor * (next_factor + 1) * (next_factor + 2)));

    if (digits(n, result) == digits(n, last_two_results[0]) &&
        digits(n, result) == digits(n, last_two_results[1])) {
      return digits(n, result);
    }
    last_two_results[1] = last_two_results[0];
    last_two_results[0] = result;
    sign = sign * -1;
    next_factor = next_factor + 2;
  }
}

int main(int argc, char* args[argc + 1]) {
  long n = strtol(args[1], 0, 10);
  printf("The first %ld digits of Pi are: %.ld\n", n, digits_of_pi(n));

  return EXIT_SUCCESS;
}
