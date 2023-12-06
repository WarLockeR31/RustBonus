fn main() {
    const N: usize = 200;
    let mut x = vec![0; N];

    println!("Array:");
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use crate::my_sort;

    #[test]
    fn test1() {
        let mut v1 = vec![1, 0, 3, 2];
        let v2 = vec![0, 1, 2, 3];
        my_sort(&mut v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test2() {
        let mut v1 = vec![0, 0, 0, -1];
        let v2 = vec![-1, 0, 0, 0];
        my_sort(&mut v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test3() {
        let mut v1 = vec![1, 1, 1, 1];
        let v2 = vec![1, 1, 1, 1];
        my_sort(&mut v1);
        assert_eq!(v1, v2);
    }
}