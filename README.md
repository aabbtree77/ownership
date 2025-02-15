Three basic ways to modify a list in Rust, to see what Rust's borrow checker is about.

```bash
rustc test_ownership.rs && ./test_ownership

Original vector: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
After modify_list_mutable: [0, 0, 0, 0, 0, 0, 0, 1, 0, 0]
After modify_list_immutable: [0, 0, 0, 0, 0, 0, 0, 1, 0, 0]
Original remains unchanged: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
After modify_list_ownership: [0, 0, 0, 0, 0, 0, 0, 1, 0, 0]
```

ChatGPT prompt:

Show me how to pass a list to a Rust function, modify its 8th element, and get back an updated list. Show me three cases: mutable, immutable with borrowing, and immutable with ownership transfer.

```rust
//test_ownership.rs

fn modify_list_mutable(vec: &mut Vec<i32>) {
    if vec.len() > 7 {
        vec[7] += 1; // Modify the 17th element
    }
}

fn modify_list_immutable(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone(); // Create a copy
    if new_vec.len() > 7 {
        new_vec[7] += 1;
    }
    new_vec
}

fn modify_list_ownership(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec; // Take ownership directly
    if new_vec.len() > 7 {
        new_vec[7] += 1;
    }
    new_vec
}

fn main() {
    let mut numbers = vec![0; 10]; // A vector with 10 zeros
    
    println!("Original vector: {:?}", numbers);

    // Case 1: Mutable reference modification
    modify_list_mutable(&mut numbers);
    println!("After modify_list_mutable: {:?}", numbers);

    // Reset numbers for the next test
    let numbers = vec![0; 10];

    // Case 2: Immutable reference modification
    let updated_numbers = modify_list_immutable(&numbers);
    println!("After modify_list_immutable: {:?}", updated_numbers);
    println!("Original remains unchanged: {:?}", numbers);

    // Case 3: Ownership transfer modification
    let numbers = vec![0; 10]; // Reset again
    let updated_numbers = modify_list_ownership(numbers);
    println!("After modify_list_ownership: {:?}", updated_numbers);
    // println!("{:?}", numbers); // Would cause an error! `numbers` is moved
}
```
