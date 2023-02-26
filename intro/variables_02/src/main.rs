fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let y:f32 = 3.0;
    println!("The value of y is: {}", y);

    let quotient = 3.0 / 2.0;
    println!("The value of quotient is: {}", quotient);

    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let truncated = 3 / 2;
    println!("The value of truncated is: {}", truncated);

    let t = true;
    let f: bool = false;
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // compound types
    let tup: (i32, f64, &str) = (500, 6.4, "text");
    let (_x, _y, _z) = tup;
    // let x = tup.0;

    let mut a = [1, 2, 3, 4, 5];
    a[0]= 50;
    println!("The value of a is: {:?}", a);

    let byte = [0; 3];
    println!("The value of byte is: {:?}", byte);

    let sum = foo(1, 2);
    println!("The value of sum is: {}", sum);

    //control flow
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}", number);

    //loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The value of result is: {}", result);

    let mut number = 3; 
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    for number in 1..4 {
        println!("{}!", number);
    }

}

fn foo(a: i32, b: i32)-> i32 {
    println!("function foo");
    println!("a: {}", a);
    println!("b: {}", b);
    a + b
}