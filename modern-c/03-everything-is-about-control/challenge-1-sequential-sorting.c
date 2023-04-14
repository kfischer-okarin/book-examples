#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void print_array(size_t size, double array[size]) {
  printf("[");
  for (int i = 0; i < size; ++i) {
    printf("%.2f", array[i]);
    if (i < size - 1) {
      printf(", ");
    }
  }
  printf("]\n");
}

void merge_sort(size_t size, double input[size], double output[size]) {
  if (size == 1) {
    output[0] = input[0];
    return;
  }

  size_t size_left = (size_t) (size / 2.0);
  double left[size_left];
  memcpy(left, input, sizeof(double) * size_left);
  double sorted_left[size_left];
  merge_sort(size_left, left, sorted_left);

  size_t size_right = size - size_left;
  double right[size_right];
  memcpy(right, input + size_left, sizeof(double) * size_right);
  double sorted_right[size_right];
  merge_sort(size_right, right, sorted_right);

  int left_index = 0;
  int right_index = 0;
  for (int i = 0; i < size; ++i) {
    if (right_index >= size_right || (left_index < size_left && sorted_left[left_index] <= sorted_right[right_index])) {
      output[i] = sorted_left[left_index];
      ++left_index;
    } else {
      output[i] = sorted_right[right_index];
      ++right_index;
    }
  }
}

int main() {
  double array1[5] = {
    5.0,
    1.2,
    -3.4,
    13.0,
    4.4,
  };

  double sorted_array1[5];

  merge_sort(5, array1, sorted_array1);

  double expected_sorted_array1[5] = {
    -3.4,
    1.2,
    4.4,
    5.0,
    13.0,
  };

  for (int i = 0; i < 5; ++i) {
    if (sorted_array1[i] != expected_sorted_array1[i]) {
      printf("Array was not sorted!\n");
      print_array(5, sorted_array1);
      return EXIT_FAILURE;
    }
  }
  printf("Array:\n");
  print_array(5, array1);
  printf("was sorted successfully:\n");
  print_array(5, sorted_array1);
  return EXIT_SUCCESS;
}
