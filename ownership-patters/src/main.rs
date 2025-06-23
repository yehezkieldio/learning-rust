struct Resource;

///
/// Drop trait ensures that the `Resource` is cleaned up properly
/// when it goes out of scope. Reminecent of C++ destructors or RAII (Resource Acquisition Is Initialization).
///
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource is being dropped");
    }
}

fn main() {
    let _res = Resource;
    // The resource will be dropped at the end of the scope
    // or when it goes out of scope.
    // Uncommenting the line above will cause the drop method to be called immediately.
    // Otherwise, it will be called automatically when `_res` goes out of scope.
    // The drop method is called automatically when the variable goes out of scope.

    // Rc gives you reference counting, allowing multiple ownership of the same resource.
    // RefCell gives you interior mutability, allowing you to mutate the resource even when it's behind an immutable reference.
    // This combination allows you to have shared ownership and mutable access to the resource.
    // Note: Be careful with using Rc and RefCell together, as it can lead to
    // runtime borrow checking errors if you try to mutate the resource while it's borrowed immutably
    // or if you try to borrow it mutably while it's already borrowed immutably.
    // This is a common pattern in Rust for managing resources that need to be shared and mutated without violating Rust's ownership rules.
    use std::cell::RefCell;
    use std::rc::Rc;

    let shared = Rc::new(RefCell::new(Resource));
    *shared.borrow_mut() = Resource; // This will not cause a double drop, as RefCell allows mutable access

    // Arc (Atomic Reference Counted) is used for thread-safe reference counting.
    // Mutex (Mutual Exclusion) is used to allow mutable access to the resource in a multi-threaded context.
    // This combination allows you to share ownership of the resource across threads
    // and mutate it safely without violating Rust's ownership rules.
    use std::sync::{Arc, Mutex};
    let shared_arc = Arc::new(Mutex::new(Resource));
    {
        let mut resource = shared_arc.lock().unwrap();
        // You can mutate the resource here
        *resource = Resource; // This will not cause a double drop, as Mutex allows mutable access
    }

    println!("End of main function");
}
