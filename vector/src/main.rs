// Arrays are usually fixed in size, but Rust provides a type called Vec that provides a flexible way to work with a list of items.

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    // debug print it to see what it looks like
    println!("{:?}", fruits);

    // we can create another vector and push items to it
    let mut fruits = Vec::new();

    fruits.push("apple");
    fruits.push("banana");

    println!("{:?}", fruits);

    // remove last element
    let last = fruits.pop();
    if let Some(last) = last {
        println!("The last fruit is {:?}", last);
    }

    // We can use the insert method to add an element at a specific index
    fruits.insert(0, "grape");
    println!("{:?}", fruits);

    // swap two elements
    fruits.swap(0, 1);
    println!("{:?}", fruits);

    // access first and last elements
    let first = fruits.first();
    if let Some(first) = first {
        println!("The first fruit is {:?}", first);
    }

    let last = fruits.last();
    if let Some(last) = last {
        println!("The last fruit is {:?}", last);
    }

    // access arbitrary element
    // Arbitrary means undetermined; not assigned a specific value."
    let second = fruits.get(1);
    // the element is already a reference, so .get gives us a double reference
    // it returns a reference to the element, to make sure we are safely accessing the element
    // without taking ownership of it or changing it

    if let Some(second) = second {
        println!("The second fruit is {:?}", second);
    }

    // init vector with a certain size
    let zeros = vec![0; 5];
    println!("{:?}", zeros);
}
