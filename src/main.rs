fn main() {
    numbers_q1();
    numbers_q2();
    numbers_q3();
    numbers_q4();
    numbers_q5();
    numbers_q6();
    numbers_q7();
    numbers_q8();
    numbers_q9();
    numbers_q10();
    numbers_q11();
    
    variables_q1();
    variables_q2();
    variables_q3();
    variables_q4();
    variables_q5();
    variables_q6();
    variables_q7();
    variables_q8();
    variables_q9();
}

fn numbers_q11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    // Integer multiplication
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("Numbers: Question 11 was a Success!");

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

fn numbers_q10() {
    use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range{ start:1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Numbers: Question 10 was a Success!");
}

fn numbers_q9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);
    println!("Numbers: Question 9 was a Success!");
    for c in 97..=122 {
        print!("{} ", c)
    }
}

fn numbers_q8() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // Why does less precision work better than more.


    println!("Numbers: Question 8 was a Success!");
}

fn numbers_q7() {
    let x = 1_000.000_1;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Numbers: Question 7 was a Success!");
}

fn numbers_q6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Numbers: Question 6 was a Success!");
}

fn numbers_q5() {
    println!("Numbers: Question 5 was a Success!");
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();

    println!("{},{}", v1, v2);
}

fn numbers_q4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Numbers: Question 4 was a Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn numbers_q3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Numbers: Question 3 was a Success!");
}

fn numbers_q2() {
    // Integer
    let _v:u16 = 38_u8 as u16;
    println!("Numbers: Question 2 was a Success!");
}

fn numbers_q1() {
    // Integer
    // If we don't explicitly assign a type to a variable,
    // then the compiler will infer one for us.

    // Remove something to make it work
    let x:i32 = 5;
    let mut _y = 5;

    _y = x;

    #[allow(unused_variables)]
    let z = 10;

    println!("Numbers: Question 1 was a Success!");
}

fn variables_q1() {
    // Binding and Mutabiblity Question
    // A variable can be used only if it has been initialized
    
    let x: i32 = 5;
    let _y:i32; // added to let the compiler know `y` is unused
    
    assert_eq!(x, 5);
    println!("Variables: Question 1 was a Success!");
}

fn variables_q2() {
    // Binding and Mutabiblity Question
    // Use mut to mark the variable as mutable
    let mut x = 1;
    x += 2;
    
    assert_eq!(x, 3);
    println!("Variables: Question 2 was a Success!");
}

fn variables_q3() {
    // Scope Question
    // A scope is the range within the program for which the item
    // is valid
    println!("Variables: Question 3 was a Success!");
    let x:i32 = 10;
    let y:i32 = 42; // added this line so that there is a `y` variable in scope
    {
        let y:i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}

fn variables_q4() {
    // Scope Question
    // A scope is the range within the program for which the item 
    // is valid
    println!("Variables: Question 4 was a Success!");
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str{
    // helper function for variables question 4
    return "hello";
}

fn variables_q5() {
    // Shadowing Question
    // You can declare a new variable with the same name as a 
    // previous variable, here we can say the first one is 
    // shadowed by the second one
    println!("Variables: Question 5 was a Success!");
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

fn variables_q6() {
    #[allow(unused_assignments)]
    let mut x:i32 = 1;
    x = 7;
    // Shadowing and re-binding
    #[allow(unused_variables)]
    let x = x;
    // x +=3; //unable to change the value of x because it is now immutable
    #[allow(unused_variables)]
    let y = 4;
    // Shadowing
    #[allow(unused_variables)]
    let y = "I can also be bound to text!";

    println!("Variables: Question 6 was a Success!");
}

fn variables_q7() {
    println!("Variables: Question 7 was a Success!");
    // Unused variables
    let _x = 1;

    #[allow(unused_variables)]
    let y = 2;
}

fn variables_q8() {
    // Destructuring
    // We can use a pattern with let to destructure a 
    // tuple to separate variables
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Variables: Question 8 was a Success!");
}

fn variables_q9() {
    // Destructuring Assignments
    // Introduced in Rust 1.59: You can now use tuple, slice,
    // and struct patterns as the left-hand side of an assignment

    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Variables: Question 9 was a Success!");
}