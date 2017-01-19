/// Performs an in-place selection sort
pub fn selection_sort(mut numbers: &mut [i32]) {
  let mut i = 0;

  while i < numbers.len() {
    let mut j = i;
    let mut min = i;

    while j < numbers.len() {
      if numbers[j] < numbers[min] {
        min = j;
      }
      j += 1;
    }

    numbers.swap(i, min);

    i += 1;
  }
}


/// Performs an in-place insertion sort
pub fn insertion_sort(mut numbers: &mut [i32]) {
  let mut i = 1;

  while i < numbers.len() {
    let mut j = i;

    while j > 0 && numbers[j-1] > numbers[j] {
      numbers.swap(j, j-1);
      j -= 1;
    }

    i += 1;
  }
}


/// Performs an in-place merge sort
pub fn merge_sort(mut numbers: &mut [i32]) {
  fn _merge_sort(mut auxilary: &mut [i32], mut numbers: &mut [i32], low: usize, high: usize) {
    if low < high {
      let size = high - low;
      let mid_point = low + size / 2;

      _merge_sort(&mut auxilary, &mut numbers, low, mid_point);
      _merge_sort(&mut auxilary, &mut numbers, mid_point + 1, high);

      let mut copy_index = low;
      while copy_index <= high {
        auxilary[copy_index] = numbers[copy_index];
        copy_index += 1;
      }

      let mut sorted_index = low;
      let mut left_index = low;
      let mut right_index = mid_point + 1;

      while sorted_index <= high {
        if left_index > mid_point || right_index <= high && auxilary[right_index] < auxilary[left_index] {
          numbers[sorted_index] = auxilary[right_index];
          right_index += 1;
        }
        else if right_index > high || auxilary[left_index] < auxilary[right_index] {
          numbers[sorted_index] = auxilary[left_index];
          left_index += 1;
        }
        sorted_index += 1;
      }
    }
  }

  let mut auxilary = vec![0; numbers.len()];
  let high_index = numbers.len() - 1;
  _merge_sort(&mut auxilary, &mut numbers, 0, high_index);
}


/// Performs an in-place heap sort
pub fn heap_sort(mut numbers: &mut [i32]) {
  fn parent(index: usize) -> usize {
    (index - 1) / 2
  }

  fn left_child(index: usize) -> usize {
    2 * index + 1
  }

  fn right_child(index: usize) -> usize {
    2 * index + 2
  }

  fn heapify(mut numbers: &mut [i32]) {
    let numbers_len = numbers.len();
    let parents_len = parent(numbers_len - 1) + 1;
    for index in (0..parents_len).rev() {
      sift_down(&mut numbers, index, numbers_len - 1);
    }
  }

  fn sift_down(mut numbers: &mut [i32], start: usize, end: usize) {
    let mut root = start;

    while left_child(root) <= end {
      let left_index = left_child(root);
      let right_index = right_child(root);
      let mut swap_candidate = root;

      if numbers[left_index] > numbers[swap_candidate] {
        swap_candidate = left_index;
      }

      if right_index <= end && numbers[right_index] > numbers[swap_candidate] {
        swap_candidate = right_index;
      }

      if swap_candidate == root {
        break;
      }

      numbers.swap(root, swap_candidate);
      root = swap_candidate;
    }
  }

  heapify(&mut numbers);

  for index in (1..numbers.len()).rev() {
    numbers.swap(index, 0);
    sift_down(&mut numbers, 0, index - 1);
  }
}


#[cfg(test)]
mod test {
  use super::*;


  fn test_sort(sort_fun: &Fn(&mut [i32])) {
    let mut numbers = vec![7, 2, 9, 10, 4, 6, 1];
    sort_fun(&mut numbers);
    assert_eq!(numbers, vec![1, 2, 4, 6, 7, 9, 10]);
  }


  #[test]
  fn test_selection_sort() {
    test_sort(&selection_sort);
  }


  #[test]
  fn test_insertion_sort() {
    test_sort(&insertion_sort);
  }


  #[test]
  fn test_merge_sort() {
    test_sort(&merge_sort);
  }


  #[test]
  fn test_heap_sort() {
    test_sort(&heap_sort);
  }

  // #[test]
  // fn test_performance() {
  //   use ndarray::prelude::*;
  //   use time::precise_time_ns;
  //   use rand::{Rng, thread_rng};
  //
  //   fn time_sort(fun: &Fn(&mut [i32]), numbers: &[i32]) -> f64 {
  //     let start_time = precise_time_ns();
  //     fun(&mut numbers.to_vec());
  //     (precise_time_ns() - start_time) as f64 / 1000.0 / 1000.0
  //   }

  //   fn generate_samples(fun: &Fn(&mut [i32]), num_samples: u32, min_size: u32,
  //                       step_size: u32) -> (Array1<f64>, Array1<f64>) {
  //     let mut rng = thread_rng();
  //     let mut sort_sizes = Array1::<f64>::zeros([num_samples as usize]);
  //     let mut sort_times = Array1::<f64>::zeros([num_samples as usize]);

  //     for sample in 0..num_samples {
  //       let sort_size = min_size + (step_size * sample);
  //       println!("Sorting {}", sort_size);

  //       let mut numbers: Vec<i32> = (0i32..sort_size as i32).collect();
  //       rng.shuffle(&mut numbers);

  //       sort_sizes[[sample as usize]] = sort_size as f64;
  //       sort_times[[sample as usize]] = time_sort(fun, &numbers[..]) as f64;
  //     }

  //     (sort_sizes, sort_times)
  //   }
  // }
}
