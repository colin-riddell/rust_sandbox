// arrays are fixed len

pub fn run(){

    let mut numbers : [i32; 5] = [1,2,3,4,5];
    numbers[0] = 20;
    println!("{}", numbers[0]);

    // get len
    println!("array length {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupes {} bytes", std::mem::size_of_val(&numbers));

    // get slices
    
    let  slice:&[i32] = &numbers[0..100];
    println!("Slice: {:?}", slice);

}