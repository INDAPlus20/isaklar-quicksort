use std::{io::{self, Read}, usize};

const CUTOFF: usize = 100;
fn main() {
    let mut line = String::with_capacity(500_000);
    io::stdin().lock().read_to_string(&mut line);
    
    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();
    
    let length = values.len(); // O(1) OPERATION
    quick_sort(&mut values, 0, length - 1);
    for e in values {
        print!("{} ", e);
    }
    print!("\n");
}

fn insert_sort(list: &mut Vec<isize>, low: usize, high: usize) {
    let mut i: usize = low + 1;

    while i < (high + 1) {
        let x = list[i];
        let mut j: i32 = i as i32 - 1;

        while j >= 0 && list[j as usize] > x {
            list[j as usize + 1] = list[j as usize];
            j -= 1;
        }
        list[(j + 1) as usize] = x;
        i += 1;
    }
}

fn quick_sort(list: &mut Vec<isize>, low: usize, high: usize) {
    if low < high {
        if high - low < CUTOFF {
            // legit saves like 0.01s
            insert_sort(list, low, high);
        } else {
            let p = partition(list, low, high);
            quick_sort(list, low, p);
            quick_sort(list, p + 1, high);
        }
        
    }
    
}

fn partition(list: &mut Vec<isize>, low: usize, high: usize) -> usize{
    let pivot = list[((low + high) as f32/2.0).floor() as usize];
    let mut i: isize = low as isize - 1;
    let mut j: usize = high + 1;
    loop{
        i += 1;
        while list[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while list[j] > pivot{
            j -= 1;
        }
        if i >= j as isize {
            return j;
        }
        list.swap(i as usize, j);
    }
}