// A dynamic array is a data structure that allocates memory as needed to store elements.
// It allows elements to be added or removed from the array.

type Node = Option<u64>;

pub struct DynamicArray {
    buf: Vec<Node>,
    cap: usize,
    pub length: usize,
}

impl DynamicArray {
    pub fn new(capacity: usize) -> Self {
        DynamicArray {
            buf: Vec::with_capacity(capacity),
            cap: capacity,
            length: 0,
        }
    }

    pub fn add(&mut self, element: u64) {
        // If current length is equal to the capacity, we resize the array
        if self.length == self.cap {
            self.resize()
        }

        self.buf.push(Some(element));
        self.length += 1;
    }

    pub fn remove(&mut self) -> Option<u64> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            // We use the pop method to remove the last element from the array
            // Flatten is used to convert the Option<u64> to u64
            self.buf.pop().flatten()
        }
    }

    pub fn get(&mut self, index: usize) -> Option<u64> {
        if index < self.length {
            self.buf[index]
        } else {
            None
        }
    }

    pub fn resize(&mut self) {
        // We increase the capacity of the array by 2
        self.cap *= 2;

        // Then we reserve the new capacity, so that the buffer can hold the new elements
        // Reserve in Rust means that we allocate memory for the buffer
        self.buf.reserve(self.cap);
    }
}

fn main() {
    let mut array = DynamicArray::new(2);
    array.add(3);
    array.add(4);
    array.add(5);

    println!("Length: {}", array.length);
    println!("Element at Index 1: {:?}", array.get(1));
    println!("Element removed: {:?}", array.remove());
    println!("Length: {}", array.length);
}
