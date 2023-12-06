fn main() {
    const N: usize = 200;
    let mut x = vec![0; N];

    for i in 0..N {
        x[i] = N as i32 - i as i32;
        print!("{:4}", x[i]);
    }
    println!();

    my_sort(&mut x);

    println!("Sorted array:");
    for i in 0..N {
        print!("{:4}", x[i]);
    }
    println!();
}

fn my_sort(x: &mut Vec<i32>) {
    quick_sort(x, 0, x.len() - 1);
}

fn quick_sort(x: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(x, low, high);
        if pivot_index > 0 {
            quick_sort(x, low, pivot_index - 1);
        }
        quick_sort(x, pivot_index + 1, high);
    }
}

fn partition(x: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = x[high];
    let mut i = low;

    for j in low..high {
        if x[j] < pivot {
            x.swap(i, j);
            i += 1;
        }
    }

    x.swap(i, high);
    i
}