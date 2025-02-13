fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;

    // Safe way 1: Using get() method
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index out of bounds"),
    }

    //Safe way 2: Check if the index is within bounds
if index < vec.len() {
        let value = vec[index];
        println!("Value at index {}: {}", index, value);
    } else {
        println!("Index out of bounds");
    }
}