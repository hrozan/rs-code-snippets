fn swap(arr: &mut Vec<i32>, i: usize, t: usize) {
    let temp = arr[i];
    arr[i] = arr[t];
    arr[t] = temp;
}

#[allow(dead_code)]
fn bubble_sort(arr: &mut Vec<i32>) {
    let mut has_swapped = true;
    while has_swapped {
        has_swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                swap(arr, i - 1, i);
                has_swapped = true
            }
        }
    }
}

#[allow(dead_code)]
fn selection_sort(arr: &mut Vec<i32>) {
    let mut min_idx: usize;
    for i in 0..arr.len() - 1 {
        min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j
            }
        }

        swap(arr, min_idx, i)
    }
}

#[test]
fn test_selection_sort() {
    let mut arr = vec![5, 2, 8, 3, 9];

    selection_sort(&mut arr);

    for i in 0..arr.len() - 1 {
        assert!(arr[i] <= arr[i + 1])
    }
}

#[test]
fn test_bubble_sort() {
    let mut arr = vec![5, 3, 2, 6, 4, 78];

    bubble_sort(&mut arr);

    for i in 0..arr.len() - 1 {
        assert!(arr[i] <= arr[i + 1])
    }
}
