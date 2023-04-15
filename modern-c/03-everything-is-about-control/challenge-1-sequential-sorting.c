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

void quick_sort(size_t size, double input[size], double output[size]) {
  if (size == 1) {
    output[0] = input[0];
    return;
  }

  size_t middle_index = (size_t) (size / 2.0);
  double pivot = input[middle_index];
  size_t size_left = 0;
  for (int i = 0; i < size; ++i) {
    if (i == middle_index) {
      continue;
    }

    if (input[i] <= pivot) {
      ++size_left;
    }e
  }
  size_t size_right = size - size_left - 1;

  double left[size_left];
  double right[size_right];
  for (int i = 0, left_index = 0, right_index = 0; i < size; ++i) {
    if (i == middle_index) {
      continue;
    }

    if (input[i] <= pivot) {
      left[left_index] = input[i];
      ++left_index;
    } else {
      right[right_index] = input[i];
      ++right_index;
    }
  }

  double sorted_left[size_left];
  if (size_left > 0) {
    quick_sort(size_left, left, sorted_left);
  }
  double sorted_right[size_right];
  if (size_right > 0) {
    quick_sort(size_right, right, sorted_right);
  }

  int output_index = 0;
  for (int i = 0; i < size_left; ++i) {
    output[output_index] = sorted_left[i];
    ++output_index;
  }
  output[output_index] = pivot;
  ++output_index;
  for (int i = 0; i < size_right; ++i) {
    output[output_index] = sorted_right[i];
    ++output_index;
  }
}

int assert_arrays_equal(size_t size, double array1[size], double array2[size]) {
  for (int i = 0; i < size; ++i) {
    if (array1[i] != array2[i]) {
      printf("Array:\n");
      print_array(size, array1);
      printf("was not equal to:\n");
      print_array(size, array2);
      return EXIT_FAILURE;
    }
  }
  return EXIT_SUCCESS;
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

  quick_sort(5, array1, sorted_array1);

  double expected_sorted_array1[5] = {
    -3.4,
    1.2,
    4.4,
    5.0,
    13.0,
  };

  if (assert_arrays_equal(5, sorted_array1, expected_sorted_array1) == EXIT_SUCCESS) {
    printf("Array sorted successfully with merge sort!\n");
    print_array(5, sorted_array1);
  } else {
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
