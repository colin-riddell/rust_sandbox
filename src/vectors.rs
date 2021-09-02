// vectors are re-sizable arrays

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4];
    numbers[0] = 20;
    println!("{}", numbers[0]);

    // get len
    println!("Vector length {}", numbers.len());

    // arrays are stack allocated
    println!("Vector occupes {} bytes", std::mem::size_of_val(&numbers));

    // get slices
    
    let  slice:&[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //push data
    numbers.push(1);


    println!("{:?}", numbers);


    for x in numbers.iter() {
        println!("Number: {}", x);
    }


    // loop and mut
    for x in numbers.iter_mut() {
    *x *=2;
    }


    println!("{:?}", numbers);


}