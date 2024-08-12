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

    // remove some elements and shift all that come after
    let mut nums = vec![1, 2, 3, 4, 5];
    let second_num = nums.remove(1);
    println!("{:?}", nums);
    println!("The second number is {:?}", second_num);

    // filter elements
    let names = vec!["John", "Jane", "Joe", "Jill", "Jack", "Alex"];

    // only keep names that start with J
    // names is moved here, so we can't use it anymore
    // into_iter() consumes the vector and returns an iterator
    // filter() creates a new iterator that only contains elements that satisfy the condition
    // chars() returns an iterator over the characters of the string, nth(0) returns the first character
    // collect() consumes the iterator and creates a new vector
    let j_names = names
        .into_iter()
        .filter(|name| name.chars().nth(0).unwrap() == 'J')
        .collect::<Vec<_>>();

    println!("{:?}", j_names);

    // remove consecutive duplicates
    let mut nums = vec![2, 2, 3, 3, 3, 4, 4, 4, 4, 1];
    nums.dedup();
    println!("{:?}", nums);

    // sort a vector
    nums.sort();
    println!("{:?}", nums);

    // sort in reverse
    nums.reverse();
    println!("{:?}", nums);

    // or
    nums.sort_by(|a, b| b.cmp(a));
    // this is done by comparing two elements at a time
    // if the closure returns Ordering::Less, it means a should come before b
    println!("{:?}", nums);

    // create a consumer iterator
    // a consumer iterator consumes the iterator and returns a value
    let mut alphabet = vec!['a', 'b', 'c', 'd', 'e'];
    println!("the firswt two letters");

    // drain() returns a draining iterator
    // it removes the elements from the vector and returns them
    for letter in alphabet.drain(..2) {
        println!("{:?}", letter);
    }

    // check the vector after draining
    println!("{:?}", alphabet);

    // split a vector
    let mut alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let alphabet2 = alphabet.split_off(3);
    // we split the vector at index 3, so the first vector will have the first 3 elements

    println!("{:?}", alphabet);
    println!("{:?}", alphabet2);

    // check if empty
    let mut nums = vec![1, 2, 3];
    println!("{:?}", nums.is_empty());

    nums.clear();
    println!("{:?}", nums.is_empty());

    // split a vector and combine it back
    let mut alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let alphabet2 = alphabet.split_off(3);

    let mut alphabet = alphabet.into_iter();
    let mut alphabet2 = alphabet2.into_iter();

    let mut alphabet3 = Vec::new();

    loop {
        match (alphabet.next(), alphabet2.next()) {
            (Some(a), Some(b)) => {
                alphabet3.push(a);
                alphabet3.push(b);
            }
            (None, None) => break,
            (Some(a), None) => alphabet3.push(a),
            (None, Some(b)) => alphabet3.push(b),
        }
    }
    println!("{:?}", alphabet3);
}
