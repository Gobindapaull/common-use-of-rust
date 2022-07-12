fn main() {
    // if else statement
    let number = 5;
    if number < 10 {
        println!("first condition was true");
    } else if number < 32 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result is {}", result);

    // for loop
    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("element are {}", ele);
    }

    for _num in 1..4 {
        println!("numbers are {}", _num);
    }
}
