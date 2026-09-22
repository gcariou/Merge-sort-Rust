use std::cmp::Ordering;

fn main() {
    // Integers
    let numbers = vec![8, 3, 5, 1, 9, 6, 2];

    let sorted_numbers = merge_sort(&numbers);
    println!("{:?}", sorted_numbers);

    // Strings
    let words = vec![
        String::from("Rust"),
        String::from("Python"),
        String::from("C++"),
        String::from("Java"),
    ];

    let sorted_words = merge_sort(&words);
    println!("{:?}", sorted_words);

    // Floats
    let floats: Vec<f64> = vec![8.2, 3.5, 1.7, 9.1];

    let sorted_floats =
        merge_sort_by(&floats, &|a, b| a.total_cmp(b));

    println!("{:?}", sorted_floats);

    // Descending order
    let sorted_higher_to_lower =
        merge_sort_by(&numbers, &|a, b| b.cmp(a));

    println!("{:?}", sorted_higher_to_lower);
}

fn merge_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    merge_sort_by(values, &|a, b| a.cmp(b))
}

fn merge_sort_by<T, F>(values: &[T], compare: &F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if values.len() <= 1 {
        return values.to_vec();
    }

    let mid = values.len() / 2;

    let left = merge_sort_by(&values[..mid], compare);
    let right = merge_sort_by(&values[mid..], compare);

    merge(&left, &right, compare)
}

fn merge<T, F>(left: &[T], right: &[T], compare: &F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mut merged = Vec::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        match compare(&left[i], &right[j]) {
            Ordering::Less | Ordering::Equal => {
                merged.push(left[i].clone());
                i += 1;
            }

            Ordering::Greater => {
                merged.push(right[j].clone());
                j += 1;
            }
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