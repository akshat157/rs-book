fn iterator_on_vec(v1: &[i32]) {
    let v1_iter = v1.iter();
    println!("Printing contents using iterator");
    for v in v1_iter {
        print!("{v} ");
    }
    println!();
}

fn consuming_adapter_on_vec(v1: &[i32]) {
    let v1_iter = v1.iter();

    let v1_sum: i32 = v1_iter.sum();

    println!("Sum = {v1_sum}");
}

fn iterator_adapter_on_vec(v1: &[i32]) {
    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    iterator_on_vec(&v2);
}

fn main() {
    let v1 = vec![1, 2, 3, 4];

    iterator_on_vec(&v1);
    consuming_adapter_on_vec(&v1);
    iterator_adapter_on_vec(&v1);
}
