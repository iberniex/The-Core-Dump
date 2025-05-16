/* merge sort algorithm: Version 1
 * Author: iberniex
 * Purpose: To becoming a programmer and segmentation fault enthusiast
 * Study: ModernC and CLRS: Introduction to algorithms and data structures for
 * reference
 *
 * integer usage: 32 bit, I think 64 bit too
 */
#include <stdio.h>

void merge(int array[], int p, int q, int r) {
  /* left and right index */
  int lindx = q - p + 1;
  int rindx = r - q;

  /* Initialize two empty arrays two sides */
  int larray[lindx], rarray[rindx];

  /* fill the arrarys from the origin array */
  for (size_t i = 0; i < lindx; i++) {
    larray[i] = array[p + i];
  }

  for (size_t i = 0; i < rindx; i++) {
    rarray[i] = array[q + i + 1];
  }

  /* initialize indices*/
  int i = 0;
  int j = 0;
  int k = p;

  /* replace first number with the number in the array */
  while (i < lindx && j < rindx) {
    if (larray[i] <= rarray[j]) {
      array[k] = larray[i];
      i++;
    } else {
      array[k] = rarray[j];
      j++;
    }
    k++;
  }

  // copy the remaining values into the array
  while (i < lindx) {
    array[k] = larray[i];
    i++;
    k++;
  }
  while (j < rindx) {
    array[k] = rarray[j];
    j++;
    k++;
  }
}

void merge_sort(int array[], int p, int r) {
  /* Base Case */
  if (p >= r) {
    return;
  }

  int q = p + (r - p) / 2;     /* middle index creation */
  merge_sort(array, p, q);     /* recursive application to left hand array */
  merge_sort(array, q + 1, r); /* recursive application to right hand array */

  merge(array, p, q, r); /* merger initiated */
}

int main(int argc, char *argv[argc + 1]) {

  int array[5] = {5, 4, 3, 2, 1};

  merge_sort(array, 0, 4);

  for (size_t i = 0; i < 5; i++) {
    printf("sorted array the %zu th is %d\n", i, array[i]);
  }
}
