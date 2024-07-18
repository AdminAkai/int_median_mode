use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut int_vec: Vec<i32> = vec![1, 1, 5, 8, 2, 3, 2, 1, 0, 1];
    int_vec.sort();
    println!("Current integers: {:?}", int_vec);
    let median = find_median(&int_vec);
    println!("Median: {median}");
    let mode = find_mode(&int_vec);
    println!("Mode: {mode}");
}

fn find_median(int_vec: &Vec<i32>) -> i32 {
    int_vec[int_vec.len() / 2]
}

fn find_mode(int_vec: &Vec<i32>) -> i32 {
    let mut common: i32 = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for i in int_vec {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;

        if counter[i] > common {
            common = *i;
        }
    }
    
    common
}
