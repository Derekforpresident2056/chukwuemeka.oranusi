fn main() {
    let numbers = [1,2,3,4,5];
    println!("original array = {:?}",numbers);

    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1);

    let slice2 = &numbers[..3];
    println!("2nd and 3rd elements sliced = {:?}",slice2);

    let slice3 = &numbers[2..];
    println!("2nd and 3rd elements sliced = {:?}",slice3);

    let slice4 = &numbers[..];
    println!("2nd and 3rd elements sliced = {:?}",slice4);
}
