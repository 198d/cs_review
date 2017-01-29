/// Performs an in-place selection sort
pub fn selection_sort<T>(mut data: &mut [T])
    where T: PartialOrd {
  let mut i = 0;

  while i < data.len() {
    let mut j = i;
    let mut min = i;

    while j < data.len() {
      if data[j] < data[min] {
        min = j;
      }
      j += 1;
    }

    data.swap(i, min);

    i += 1;
  }
}


/// Performs an in-place insertion sort
pub fn insertion_sort<T>(mut data: &mut [T])
    where T: PartialOrd {
  let mut i = 1;

  while i < data.len() {
    let mut j = i;

    while j > 0 && data[j-1] > data[j] {
      data.swap(j, j-1);
      j -= 1;
    }

    i += 1;
  }
}


/// Performs an in-place merge sort
pub fn merge_sort<T>(mut data: &mut [T])
    where T: PartialOrd + Copy {
  fn _merge_sort<T>(mut auxilary: &mut [T], mut data: &mut [T], low: usize, high: usize)
      where T: PartialOrd + Copy {
    if low < high {
      let size = high - low;
      let mid_point = low + size / 2;

      _merge_sort(&mut auxilary, &mut data, low, mid_point);
      _merge_sort(&mut auxilary, &mut data, mid_point + 1, high);

      let mut copy_index = low;
      while copy_index <= high {
        auxilary[copy_index] = data[copy_index];
        copy_index += 1;
      }

      let mut sorted_index = low;
      let mut left_index = low;
      let mut right_index = mid_point + 1;

      while sorted_index <= high {
        if left_index > mid_point || right_index <= high && auxilary[right_index] < auxilary[left_index] {
          data[sorted_index] = auxilary[right_index];
          right_index += 1;
        }
        else if right_index > high || auxilary[left_index] < auxilary[right_index] {
          data[sorted_index] = auxilary[left_index];
          left_index += 1;
        }
        sorted_index += 1;
      }
    }
  }

  if data.len() > 0 {
    let mut auxilary = vec![data[0]; data.len()];
    let high_index = data.len() - 1;
    _merge_sort(&mut auxilary, &mut data, 0, high_index);
  }
}


/// Performs an in-place heap sort
pub fn heap_sort<T>(mut data: &mut [T])
    where T: PartialOrd {
  fn parent(index: usize) -> usize {
    (index - 1) / 2
  }

  fn left_child(index: usize) -> usize {
    2 * index + 1
  }

  fn right_child(index: usize) -> usize {
    2 * index + 2
  }

  fn heapify<T>(mut data: &mut [T])
      where T: PartialOrd {
    let data_len = data.len();
    let parents_len = parent(data_len - 1) + 1;
    for index in (0..parents_len).rev() {
      sift_down(&mut data, index, data_len - 1);
    }
  }

  fn sift_down<T>(mut data: &mut [T], start: usize, end: usize)
      where T: PartialOrd {
    let mut root = start;

    while left_child(root) <= end {
      let left_index = left_child(root);
      let right_index = right_child(root);
      let mut swap_candidate = root;

      if data[left_index] > data[swap_candidate] {
        swap_candidate = left_index;
      }

      if right_index <= end && data[right_index] > data[swap_candidate] {
        swap_candidate = right_index;
      }

      if swap_candidate == root {
        break;
      }

      data.swap(root, swap_candidate);
      root = swap_candidate;
    }
  }

  heapify(&mut data);

  for index in (1..data.len()).rev() {
    data.swap(index, 0);
    sift_down(&mut data, 0, index - 1);
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
