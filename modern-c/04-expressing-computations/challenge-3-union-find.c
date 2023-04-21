#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void initialize_parent(size_t size, size_t parent[size]) {
  for (size_t i = 0; i < size; ++i) {
    parent[i] = SIZE_MAX;
  }
}

bool is_root(size_t parent[], size_t index) {
  return parent[index] == SIZE_MAX;
}

size_t find(size_t parent[], size_t index) {
  size_t result = index;

  while (true) {
    if (parent[result] == SIZE_MAX) {
      break;
    }

    result = parent[result];
  }

  return result;
}

void find_replace(size_t parent[], size_t element, size_t new_parent) {
  size_t old_parent;
  while (true) {
    old_parent = parent[element];
    parent[element] = new_parent;
    if (old_parent == SIZE_MAX) {
      break;
    }
    element = old_parent;
  }
}

void find_compress(size_t parent[], size_t element) {
  size_t root = find(parent, element);
  find_replace(parent, element, root);
  parent[root] = SIZE_MAX;
}

void Union(size_t parent[], size_t element1, size_t element2) {
  find_compress(parent, element1);
  find_replace(parent, element2, find(parent, element1));
}

void assert_parent(size_t parent[], size_t element, size_t expected_parent) {
  if (parent[element] != expected_parent) {
    printf("Expected parent of element %ld to be %ld, but it was %ld\n",
           element,
           expected_parent,
           parent[element]);
  }
}

int main() {
  size_t base_set_size = 10;
  size_t parent[base_set_size];

  puts("---initialize_parent---");
  initialize_parent(base_set_size, parent);

  for (size_t i = 0; i < base_set_size; ++i) {
    if (find(parent, i) != i) {
      printf("Element %ld was not root but had parent %ld\n",
             i,
             parent[i]);
    }
  }

  puts("---find---");
  // 3 -> 2 -> 4
  // 6 -> 1 -> 9 -> 7
  // 5 -> 0
  // 8
  parent[3] = 2;
  parent[2] = 4;

  parent[6] = 1;
  parent[1] = 9;
  parent[9] = 7;

  parent[5] = 0;

  if (find(parent, 3) != 4) {
    printf("Root of element 3 was not 4 but %ld",
           find(parent, 3));
  }
  if (find(parent, 6) != 7) {
    printf("Root of element 6 was not 7 but %ld",
           find(parent, 6));
  }

  puts("---find_replace---");
  find_replace(parent, 3, 8);
  // 2, 3, 4 -> 8
  // 6 -> 1 -> 9 -> 7
  // 5 -> 0

  assert_parent(parent, 3, 8);
  assert_parent(parent, 2, 8);
  assert_parent(parent, 4, 8);

  puts("---find_compress---");
  find_compress(parent, 6);
  // 2, 3, 4 -> 8
  // 1, 6, 9 -> 7
  // 5 -> 0

  assert_parent(parent, 1, 7);
  assert_parent(parent, 6, 7);
  assert_parent(parent, 9, 7);
  assert_parent(parent, 7, SIZE_MAX);

  puts("---Union---");
  parent[0] = 1;
  // 2, 3, 4 -> 8
  // 5 -> 0 -> 1 /  1, 6, 9 -> 7

  Union(parent, 0, 3);
  // 5 -> 0 / 2, 4 -> 8 / 0, 1, 3, 6, 8, 9 -> 7

  assert_parent(parent, 5, 0);
  assert_parent(parent, 2, 8);
  assert_parent(parent, 4, 8);
  assert_parent(parent, 0, 7);
  assert_parent(parent, 1, 7);
  assert_parent(parent, 3, 7);
  assert_parent(parent, 6, 7);
  assert_parent(parent, 8, 7);
  assert_parent(parent, 9, 7);

  return EXIT_SUCCESS;
}
