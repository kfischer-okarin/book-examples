#include <stdio.h>
#include <stdlib.h>
#include <math.h>

const double EPSILON = 0.00001;

double derivative(double f(double), double x) {
  return (f(x + EPSILON) - f(x - EPSILON)) / (2 * EPSILON);
}

int main(int argc, char* args[argc + 1]) {
  double x = strtod(args[1], 0);
  double result = derivative(*sin, x);
  printf("The derivative of sin(%.2f) is %.5f\n", x, result);
  printf("The cos is %.5f\n", cos(x));

  return EXIT_SUCCESS;
}
