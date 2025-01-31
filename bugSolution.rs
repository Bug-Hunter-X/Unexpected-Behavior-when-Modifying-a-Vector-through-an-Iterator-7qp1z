fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    for item in vec.iter_mut() {
        *item = 10;
    }

    println!("After: {:?}", vec);
}