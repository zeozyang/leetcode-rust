fn main() {
    let mut arr = vec![-3, 5, -34, 2, 8, -31, 0, 99, 7, 2, 2];
    heap_sort(&mut arr);
    println!("{:?}", arr);
}

fn heap_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() || arr.len() < 2 {
        return;
    }
    for i in 1..arr.len() {
        heap_insert(arr, i);
    }

    let mut heap_size = arr.len();
    heap_size -= 1;
    arr.swap(0, heap_size);
    while heap_size > 0 {
        heaping(arr, heap_size);
        heap_size -= 1;
        arr.swap(0, heap_size);
    }
}

fn heap_insert(arr: &mut Vec<i32>, mut index: usize) {
    while index != 0 && arr[index] > arr[(index - 1) / 2] {
        arr.swap((index - 1) / 2, index);
        index = (index - 1) / 2;
    }
}

fn heaping(arr: &mut Vec<i32>, heap_size: usize) {
    let mut root = 0;
    let mut l = root * 2 + 1;
    while l < heap_size {
        let r = l + 1;
        let mut largest = if r < heap_size && arr[r] > arr[l] {
            r
        } else {
            l
        };
        largest = if arr[largest] > arr[root] {
            largest
        } else {
            root
        };

        if largest == root {
            break;
        } else {
            arr.swap(largest, root);
            root = largest;
            l = root * 2 + 1;
        }
    }
}
