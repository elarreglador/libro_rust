fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Todo el array: {:?}", a);

    let slice = &a[1..3];

    println!("Slice del array {:?}", slice);

    assert_eq!(slice, &[2, 3]);

}
