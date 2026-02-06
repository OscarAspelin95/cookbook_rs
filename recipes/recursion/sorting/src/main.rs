/// Recursive sort.
///
/// NOTE - this is by no means the most effective solution,
/// but it illustrates recursive calls which is the point.
fn sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    // Potential improvement - Vec::with_capacity(v.len() / 2)
    // for approximating what capacity we need.
    let mut left: Vec<T> = vec![];
    let mut right: Vec<T> = vec![];

    // v is non-empty, so this is safe.
    // It is, however catastrophic if v is already sorted.
    let last = v.pop().expect("Should never happen");

    for num in v {
        if num <= last {
            left.push(num);
        } else {
            right.push(num);
        }
    }

    let mut v_sorted: Vec<T> = Vec::with_capacity(left.len() + right.len() + 1);

    // Recursive call(s).
    let v_left = sort(left);
    let v_right = sort(right);

    v_sorted.extend(v_left);
    v_sorted.push(last);
    v_sorted.extend(v_right);

    v_sorted
}

fn main() {
    assert_eq!(sort(vec![3, 2, 1]), vec![1, 2, 3]);
    assert_eq!(sort(vec![10]), vec![10]);
    assert_eq!(sort(vec![10, -10, 1, -1]), vec![-10, -1, 1, 10]);
    assert_eq!(sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]); // worst case scenario.
}
