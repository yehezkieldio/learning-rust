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

    println!("End of main function");
}
