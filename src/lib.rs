pub fn bubble_sort(arr: &mut Vec<i32>) {
    let mut has_swap = true;
    while has_swap {
        has_swap = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
                has_swap = true;
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut arr: Vec<i32>  = vec![8, 3, 5, 2, 7, 1];

    bubble_sort(&mut arr);

    for i in 0..arr.len() - 1 {
        assert!(arr[i] <= arr[i + 1]);
    }
}
