use std::collections::HashMap;

fn main() {
    let mut list = vec![10,21,3,4,5,56,6,45,4,43,5,6,76,5,4,1,2,5,5,34,2,13,12,34];
    println!("{list:?}");

    list.sort();

    // println!("sorted: {list:?}");

    // median calculation
    let median = if list.len().is_multiple_of(2) {
        (list[list.len()/2] + list[list.len()/2 - 1]) as f32 / 2.0
    } else {
        list[list.len()/2] as f32
    };

    let mut freq_hashmap = HashMap::new();

    for i in &list {
        let count = freq_hashmap.entry(*i).or_insert(0);

        *count += 1;
    }


    let mut max_freq: u32 = 0;
    let mut mode = 0;

    // mode calculation
    for (num, freq) in &freq_hashmap {
       if *freq > max_freq {
           max_freq = *freq;
           mode = *num;
       }
    }

    // println!("{freq_hashmap:?}");

    // mean calculation
    let sum: i32 = list.iter().sum();
    let mean = (sum as f32) / (list.len() as f32);

    println!("Mean: {mean}, Mode: {mode}, Median: {median}");
}

