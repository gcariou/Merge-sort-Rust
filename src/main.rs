fn main() {
    let numbers = vec![8, 3, 5, 1, 9, 6, 2];
    let sorted_numbers = merge_sort(&numbers);

    println!("{:?}", sorted_numbers);

    let words = vec![
        String::from("Rust"),
        String::from("Python"),
        String::from("C++"),
        String::from("Java"),
    ];

    let sorted_words = merge_sort(&words);

    println!("{:?}", sorted_words);
}

fn merge_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    if values.len() <= 1 {
        return values.to_vec();
    }

    let mid = values.len() / 2;
    let left = merge_sort(&values[..mid]);
    let right = merge_sort(&values[mid..]);
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    while i < left.len() {
        merged.push(left[i].clone());
        i += 1;
    }
    
    while j < right.len() {
        merged.push(right[j].clone());
        j += 1;
    }

    merged
}