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

    // C++ has lifetimes, but they're implicit. Rust has explicit lifetimes, which are a way to track how long references to data are valid.
    // This helps prevent dangling references and ensures memory safety.

    // Understand lifetime variance (covariant, contravariant, invariant) in Rust.
    // Covariant: If `A` is a subtype of `B`, then `&A` is a subtype of `&B`.
    // Contravariant: If `A` is a subtype of `B`, then `fn(A) -> C` is a subtype of `fn(B) -> C`.
    // Invariant: If `A` is a subtype of `B`, then `&mut A` is not a subtype of `&mut B`.
    // This is a fundamental concept in Rust's type system and helps ensure that references are used safely.

    // Pin is a way to ensure that a value is not moved in memory, which is useful for self-referential structs.
    // It is similar to C++'s std::unique_ptr or std::shared_ptr, but with a focus on ensuring that the value is not moved after it has been pinned.
    // Self-referential structs are structs that contain references to themselves, which can lead to issues if the struct is moved in memory.
    use std::pin::Pin;
    struct MyStruct {
        value: i32,
    }
    impl MyStruct {
        fn new(value: i32) -> Self {
            MyStruct { value }
        }

        fn get_value(self: Pin<&Self>) -> i32 {
            self.value
        }
    }
    let my_struct = MyStruct::new(42);
    let pinned_struct = Pin::new(&my_struct);
    // The compiler ensures that `pinned_struct` cannot be moved in memory,
    // which allows us to safely call methods that rely on the struct not being moved.
    // This is useful for self-referential structs or when you need to ensure that the
    println!("Pinned value: {}", pinned_struct.get_value());

    println!("End of main function");
}
