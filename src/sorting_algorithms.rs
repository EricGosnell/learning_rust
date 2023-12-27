use std::cmp;
use rand::Rng;
use std::time::Instant;
use graplot::Plot;

#[allow(dead_code)]

/* Sorting algorithm source code from https://github.com/TheAlgorithms/Rust/tree/master/src/sorting */
fn is_sorted<T>(arr: &[T]) -> bool where T: cmp::PartialOrd, {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/* Merge Sort */
fn merge_sort<T: Ord + Copy>(arr: &mut [T]) { // Top down
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        // Sort the left half recursively.
        merge_sort(&mut arr[..mid]);
        // Sort the right half recursively.
        merge_sort(&mut arr[mid..]);
        // Combine the two halves.
        merge(arr, mid);
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // Create temporary vectors to support the merge.
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // Indexes to track the positions while merging.
    let mut l = 0;
    let mut r = 0;

    for v in arr {
        // Choose either the smaller element, or from whichever vec is not exhausted.
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

/* Quick Sort */
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, len - 1);
    }
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        if p > 0 {
            _quick_sort(arr, lo, p - 1);
        }
        _quick_sort(arr, p + 1, hi);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j - 1] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j - 1 {
            break;
        } else if arr[i] == arr[j - 1] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j - 1);
        }
    }
    arr.swap(i, pivot);
    i
}

/* Heap Sort */
fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    // Convert to max heap
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

/* Shell Sort */
fn shell_sort<T: Ord + Copy>(values: &mut [T]) {
    // shell sort works by swiping the value at a given gap and decreasing the gap to 1
    fn insertion<T: Ord + Copy>(values: &mut [T], start: usize, gap: usize) {
        for i in ((start + gap)..values.len()).step_by(gap) {
            let val_current = values[i];
            let mut pos = i;
            // make swaps
            while pos >= gap && values[pos - gap] > val_current {
                values[pos] = values[pos - gap];
                pos -= gap;
            }
            values[pos] = val_current;
        }
    }

    let mut count_sublist = values.len() / 2; // makes gap as long as half of the array
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2; // makes gap as half of previous
    }
}

/* Insertion Sort */
fn insertion_sort(arr: &mut Vec<i32>, left: usize, right: usize) -> &Vec<i32> {
    for i in (left + 1)..(right + 1) {
        let temp = arr[i];
        let mut j = (i - 1) as i32;

        while j >= (left as i32) && arr[j as usize] > temp {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = temp;
    }
    arr
}

/* Tim Sort */
fn tim_sort(arr: &mut Vec<i32>, n: usize) {
    let min_run = min_run_length(32);

    let mut i = 0;
    while i < n {
        insertion_sort(arr, i, cmp::min(i + 32 - 1, n - 1));
        i += min_run;
    }

    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = cmp::min(left + 2 * size - 1, n - 1);
            if mid < right {
                tim_merge(arr, left, mid, right);
            }

            left += 2 * size;
        }
        size *= 2;
    }
}

fn min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= 32 {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn tim_merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) -> &Vec<i32> {
    let len1 = m - l + 1;
    let len2 = r - m;
    let mut left = vec![0; len1];
    let mut right = vec![0; len2];

    left[..len1].clone_from_slice(&arr[l..(len1 + l)]);

    for x in 0..len2 {
        right[x] = arr[m + 1 + x];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        arr[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < len2 {
        arr[k] = right[j];
        k += 1;
        j += 1;
    }
    arr
}

/* Radix Sort */
// fn radix_sort(arr: &mut Vec<i32>) {
//     let max: usize = match arr.iter().max() {
//         Some(&x) => x as usize,
//         None => return,
//     };
//     // Make radix a power of 2 close to arr.len() for optimal runtime
//     let radix = arr.len().next_power_of_two();
//     // Counting sort by each digit from least to most significant
//     let mut place = 1;
//     while place <= max {
//         let digit_of = |x| x as usize / place % radix;
//         // Count digit occurrences
//         let mut counter = vec![0; radix];
//         for &x in arr.iter() {
//             counter[digit_of(x)] += 1;
//         }
//         // Compute last index of each digit
//         for i in 1..radix {
//             counter[i] += counter[i - 1];
//         }
//         // Write elements to their new indices
//         for &x in arr.to_owned().iter().rev() {
//             counter[digit_of(x)] -= 1;
//             arr[counter[digit_of(x)]] = x;
//         }
//         place *= radix;
//     }
// }

fn performance(sort_type: String, vector_size :usize) -> f64 {
    let mut unsorted_vector: Vec<i32> = vec![];
    for _i in 0..vector_size {
        unsorted_vector.push(rand::thread_rng().gen_range(1..=10))
    }
    // println!("Input array: {:?}", unsorted_vector);
    match sort_type.as_str() {
        "merge" => {
            let start_time = Instant::now();
            for _i in 0..100 {
                merge_sort(&mut unsorted_vector);
            }
            let elapsed_time = (Instant::now() - start_time) / 100;
            elapsed_time.as_nanos() as f64
        },
        "quick" => {
            let start_time = Instant::now();
            for _i in 0..100 {
                quick_sort(&mut unsorted_vector);
            }
            let elapsed_time = (Instant::now() - start_time) / 100;
            elapsed_time .as_nanos() as f64
        },
        "heap" => {
            let start_time = Instant::now();
            for _i in 0..100 {
                heap_sort(&mut unsorted_vector);
            }
            let elapsed_time = (Instant::now() - start_time) / 100;
            elapsed_time.as_nanos() as f64
        },
        "shell" => {
            let start_time = Instant::now();
            for _i in 0..100 {
                shell_sort(&mut unsorted_vector);
            }
            let elapsed_time = (Instant::now() - start_time) / 100;
            elapsed_time.as_nanos() as f64
        },
        "tim" => {
            let start_time = Instant::now();
            for _i in 0..100 {
                tim_sort(&mut unsorted_vector, vector_size);
            }
            let elapsed_time = (Instant::now() - start_time) / 100;
            elapsed_time.as_nanos() as f64
        },
        // "radix" => {
        //     let start_time = Instant::now();
        //     for _i in 0..100 {
        //         radix_sort(&mut unsorted_vector);
        //     }
        //     let elapsed_time = (Instant::now() - start_time) / 100;
        //     elapsed_time.as_nanos() as f64
        // }
        _ => {0.}
    }
}

fn main() {
    let log2x = [0.,1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
    let mut merge_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut quick_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut heap_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut shell_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut tim_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    // let mut radix_time:[f64;11] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];

    for i in 0..=10 {
        merge_time[i] = (performance("merge".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
        quick_time[i] = (performance("quick".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
        heap_time[i] = (performance("heap".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
        shell_time[i] = (performance("shell".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
        tim_time[i] = (performance("tim".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
        // radix_time[i] = (performance("radix".parse().unwrap(), 2_i32.pow(i as u32) as usize))/(2_i32.pow(i as u32) as f64);
    }

    println!("Merge sort time{:?}", merge_time);
    println!("Quick sort time{:?}", quick_time);
    println!("Heap sort time{:?}", heap_time);
    println!("Shell sort time{:?}", shell_time);
    println!("Tim sort time{:?}", tim_time);
    // println!("Radix sort time{:?}", radix_time);

    let mut plot = Plot::new((log2x, merge_time, "r-o"));
    plot.add((log2x, quick_time, "g-o"));
    plot.add((log2x, heap_time, "b-o"));
    plot.add((log2x, shell_time, "c-o"));
    plot.add((log2x, tim_time, "y-o"));
    // plot.add((log2x, radix_time, "c-o"));
    plot.show();
}
/* TODO: enum for merge, quick, heap, shell, tim, and radix */
