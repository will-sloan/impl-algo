// mod merge_sort;
// mod selection_sort;
// mod insertion_sort;
//mod quick_sort;
mod binary_search;
fn main() {
    let a: Vec<i32> = test_sort();
    let b: i32 = a[6].clone();
    binary_search::binary::binary_search(a, b);
}

fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    while !left.is_empty() || !right.is_empty() {
        if !left.is_empty() && !right.is_empty() {
            //                 println!("In both");
            if left[0] <= right[0] {
                result.push(left.remove(0));
            } else {
                result.push(right.remove(0));
            }
        } else if !left.is_empty() {
            result.append(left);
        } else {
            result.append(right);
        }
    }
    result
}
fn sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let mut left: Vec<i32> = arr[..mid].to_vec();
    let mut right: Vec<i32> = arr[mid..].to_vec();
    left = sort(left);
    right = sort(right);
    merge(&mut left, &mut right)
}

fn test_sort() -> Vec<i32>{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = Vec::new();
    for _ in 0..10 {
        arr.push(rng.gen_range(0, 100));
    }
    println!("{:?}", arr);
    arr = sort(arr);
    println!("{:?}", arr);
    arr
}
