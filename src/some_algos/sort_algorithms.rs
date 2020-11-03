pub fn bubble_sort_0(arr: &mut [i32]) {
    for n in (0..arr.len()).rev() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_sort_1(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
    }
}

pub fn bubble_sort_2(arr: &mut [i32]) {

}