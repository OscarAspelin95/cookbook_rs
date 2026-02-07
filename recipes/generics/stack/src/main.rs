mod stack;
use stack::Stack;

fn main() {
    // -----------------
    let mut stack: Stack<usize> = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    let num_elements = 5;
    (1..num_elements + 1).for_each(|i| {
        stack.push(i);
        assert_eq!(stack.len(), i);
    });

    let first = stack.pop();
    assert_eq!(first, Some(5));

    println!("stack #1: {}", &stack);

    // -----------------
    let mut stack_with_capacity: Stack<usize> = Stack::with_capacity(5);
    assert_eq!(stack_with_capacity.capacity(), 5);

    stack_with_capacity.push(3);
    stack_with_capacity.push(1);
    stack_with_capacity.push(2);
    assert_eq!(stack_with_capacity.peek(), Some(&3));
    stack_with_capacity.sort();
    assert_eq!(stack_with_capacity.peek(), Some(&1));

    println!("stack #2 (sorted): {}", &stack_with_capacity)
}
