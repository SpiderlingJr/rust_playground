pub fn basics() {
    let x = 5;
    println!("The value of x is: {}", x);


    //Illegal: Don't mutate immutable variable. Fix: let mut x
    //x: u32 = 6;


    // This works, because spaces becomes a new variable.
    let spaces = "     ";
    let spaces = spaces.len();

    // This breaks, because spaces is assigned a string value.
    // Cant change string to int later.
    //let spaces = "     ";
    //spaces = spaces.len()


    // This breaks, as rust isnt able to guess the resulting datatype
    //let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    // Compund Types
    // Storing multiple values in one type. Primitive: tuples and arrays

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // deconstructing
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Accessing indizes:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    // Every element must have same type.
    let a = [1, 2, 3, 4, 5];

    // Accessing indizes:
    let first = a[0];
    let second = a[1];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    //let a = [3; 5]; --> [3,3,3,3,3];


    another_function(a[0]);


    //flop();
    //if_and_else();

    loop_count();

}

fn another_function(x: i32) -> i32 {
    println!("The value of x is {}", x);
    return 1;
}

fn flop() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}

fn if_and_else() {
    let number = 3;
    if number < 5 {
        println!("True!");
    } else {
        println!("False!");
    }
}

fn conditional_let(condition: bool) -> i32 {
    let number = if condition { 5 } else { 6 };
    return number;
}

fn loop_count() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result)
}

fn for_each() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Value: {}", element)
    }
}