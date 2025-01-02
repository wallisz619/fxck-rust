fn main() {
    // let mut x = 5;
    // let x = 2;
    // println!("the value of x is {}", x);

    // let x = 6;
    // println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // let float1= 0.1;
    // let float2: f32 = 0.1;

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}, ", x, y, z);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // array
    let arr = [1,2,3,4,5];
    println!("{}, ", arr[4]);

    let x = another_function(5); // argument

    print!("value is {}", x);

    // match
    // if else


    // loop
    // loop {
    //     // print!("{}", 5)
    // }

    // for 循环
    let arr2 = [10, 20, 30, 40, 50, 60];
    for element in arr2.iter() { 
        println!("the value is {}", element);
    }

    // Range ..
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");


}


fn another_function(x: i32) -> i32 { // parameter
    return x + 5;
}
