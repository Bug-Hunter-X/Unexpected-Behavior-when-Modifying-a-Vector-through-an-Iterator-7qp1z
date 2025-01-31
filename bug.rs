fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    let mut iter = vec.iter_mut();
    *iter.next().unwrap() = 10;

    println!("After: {:?}", vec);
}