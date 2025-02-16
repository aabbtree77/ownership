use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::thread;
use tokio::runtime::Runtime;
use gc::{Gc, GcCell};

// Case 1: Mutable Borrow - Safe lock: You open, modify, and close it.
fn modify_list_mut(vec: &mut Vec<i32>) {
    println!("Accessing list via mutable reference...");
    if vec.len() > 7 {
        vec[7] = 42;
    }
    println!("Modified list inside function: {:?}", vec);
}

fn case_mutable() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Before modification: {:?}", numbers);
    modify_list_mut(&mut numbers);
    println!("After modification: {:?}", numbers);
}

// Case 2: Immutable Borrow - Hotel room: You can look but not change anything.
fn modify_list_immutable(vec: &Vec<i32>) -> Vec<i32> {
    println!("Cloning list, original remains untouched...");
    let mut new_vec = vec.clone();
    if new_vec.len() > 7 {
        new_vec[7] = 42;
    }
    new_vec
}

fn case_immutable() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Before modification: {:?}", numbers);
    let modified_numbers = modify_list_immutable(&numbers);
    println!("After modification (new list): {:?}", modified_numbers);
}

// Case 3: Ownership Transfer - Moving van: You give away the whole thing.
fn modify_list_owned(mut vec: Vec<i32>) -> Vec<i32> {
    println!("Taking full ownership of the list...");
    if vec.len() > 7 {
        vec[7] = 42;
    }
    vec
}

fn case_owned() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Before transfer: {:?}", numbers);
    let new_numbers = modify_list_owned(numbers);
    println!("After transfer: {:?}", new_numbers);
}

// Case 4: Box - Storage unit: You keep ownership but store it elsewhere.
fn case_box() {
    let numbers = Box::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("Stored in a Box: {:?}", numbers);
}

// Case 5: Rc - Shared Library: Multiple users can read it but not modify it.
fn case_rc() {
    let numbers = Rc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let shared = Rc::clone(&numbers);
    println!("Original Rc: {:?}, Cloned Rc: {:?}", numbers, shared);
}

// Case 6: Arc - Secure Webpage: Many threads access read-only content.
fn case_arc() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let numbers_clone = Arc::clone(&numbers);
    thread::spawn(move || {
        println!("Thread accessing Arc: {:?}", numbers_clone);
    }).join().unwrap();
}

// Case 7: RefCell - Restroom: Multiple access, but only one can modify at a time.
fn case_refcell() {
    let numbers = RefCell::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    {
        let mut borrow_mut = numbers.borrow_mut();
        borrow_mut[7] = 42;
        println!("Modified inside RefCell: {:?}", borrow_mut);
    }
    println!("After RefCell modification: {:?}", numbers.borrow());
}

// Case 8: Mutex - Safe Lock: One at a time access in multi-threaded scenarios.
fn case_mutex() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let cloned = Arc::clone(&numbers);
    thread::spawn(move || {
        let mut lock = cloned.lock().unwrap();
        lock[7] = 42;
        println!("Modified inside Mutex: {:?}", lock);
    }).join().unwrap();
    println!("After Mutex modification: {:?}", numbers.lock().unwrap());
}

// Case 9: Async - Server request queue: Processing data asynchronously.
async fn modify_list_async(vec: Vec<i32>) -> Vec<i32> {
    println!("Async processing started...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let mut new_vec = vec;
    if new_vec.len() > 7 {
        new_vec[7] = 42;
    }
    println!("Async processing completed.");
    new_vec
}

fn case_async() {
    let rt = Runtime::new().unwrap();
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Before async: {:?}", numbers);
    let modified = rt.block_on(modify_list_async(numbers));
    println!("After async: {:?}", modified);
}

// Case 10: Async Lifetime - Managing a connection that outlives a function.
async fn modify_list_async_lifetime<'a>(vec: &'a mut Vec<i32>) {
    println!("Async lifetime modification started...");
    if vec.len() > 7 {
        vec[7] = 42;
    }
    println!("Async lifetime modification completed: {:?}", vec);
}

fn case_async_lifetime() {
    let rt = Runtime::new().unwrap();
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Before async lifetime: {:?}", numbers);
    rt.block_on(modify_list_async_lifetime(&mut numbers));
    println!("After async lifetime: {:?}", numbers);
}

// Case 11: Gc - Smart Trash Bin. It will be cleaned when unused.
fn case_gc() {
    let vec = Gc::new(GcCell::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("[🗑️ Smart Trash Bin] Before: {:?}", vec.borrow());
    modify_list_gc(vec.clone());
    println!("[🗑️ Smart Trash Bin] After: {:?}", vec.borrow());
}
fn modify_list_gc(vec: Gc<GcCell<Vec<i32>>>) {
    let mut borrow_mut = vec.borrow_mut();
    if borrow_mut.len() > 7 {
        borrow_mut[7] *= 2;
    }
}

fn main() {

    println!("");
    case_mutable();
    println!("--------------------------------");
    case_immutable();
    println!("--------------------------------");
    case_owned();
    println!("--------------------------------");
    case_box();
    println!("--------------------------------");
    case_rc();
    println!("--------------------------------");
    case_arc();
    println!("--------------------------------");
    case_refcell();
    println!("--------------------------------");
    case_mutex();
    println!("--------------------------------");
    case_async();
    println!("--------------------------------");
    case_async_lifetime();
    println!("--------------------------------");
    case_gc();
    println!("--------------------------------");

}

