fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe methods to modify the vector's elements
    v[0] = 4; //Safe access
    println!("v[0]: {}", v[0]);
} 