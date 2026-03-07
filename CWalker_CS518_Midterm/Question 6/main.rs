/// A small abstraction that does not rely on a specific algorithm.
///
/// This keeps the public design open for extension such as
/// MergeSort, HeapSort, etc. later without changing caller code.
pub trait Sorter<T: Ord> {
    fn sort(&self, data: &mut [T]);
}

/// Classic textbook Quicksort using recursion and the Lomuto partition scheme.
/// The last element is chosen as the pivot.
/// This keeps the implementation close to what is commonly shown in data structures books.
/// It is simple and clear, though not always the most efficient pivot strategy.
pub struct QuickSort;

impl<T: Ord> Sorter<T> for QuickSort {
    fn sort(&self, data: &mut [T]) {
        quicksort(data);
    }
}

/// Public convenience function.
pub fn quicksort<T: Ord>(data: &mut [T]) {
    quicksort_recursive(data);
}

fn quicksort_recursive<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);

    // After partitioning, the slice must be divided into left and right regions.
    // Rust requires proof that these two mutable regions do not overlap.
    // `split_at_mut` provides that guarantee safely.
    // This makes recursive sorting of both halves possible without unsafe code.
    let (left, right_with_pivot) = data.split_at_mut(pivot_index);

    // `right_with_pivot[0]` is the pivot itself after partitioning.
    // We skip it and recurse only on the right side after the pivot.
    let (_, right) = right_with_pivot
        .split_first_mut()
        .expect("right_with_pivot always contains the pivot");

    quicksort_recursive(left);
    quicksort_recursive(right);
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let pivot_index = data.len() - 1;
    let mut store_index = 0;

    // IMPORTANT RUST COMMENT:
    //
    // In many textbook versions, the pivot is stored as a direct reference.
    // That approach is awkward in Rust because the slice is being mutated during partitioning.
    // Keeping track of the pivot by index avoids borrow conflicts while staying fully safe.
    // This is one of the main ways Quicksort must be adapted to fit Rust well.

    for i in 0..pivot_index {
        if data[i] <= data[pivot_index] {
            data.swap(i, store_index);
            store_index += 1;
        }
    }

    data.swap(store_index, pivot_index);
    store_index
}

fn main() {
    let mut numbers = [33, 10, 55, 71, 29, 3, 18, 42];
    let sorter = QuickSort;
    sorter.sort(&mut numbers);

    println!("Sorted numbers: {:?}", numbers);

    let mut words = ["pear", "apple", "orange", "banana"];
    sorter.sort(&mut words);

    println!("Sorted words: {:?}", words);
}