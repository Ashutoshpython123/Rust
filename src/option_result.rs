pub fn option_result() {
    // pub enum Option<T> {
    //     Some(T), ->  Represents a value of type T
    //     None  -> Represents the absence of a value
    // }

    // pub enum Result<T, E> {
    //     Ok(T),  ->Represents a successful operation with a value of type T
    //     Err(E), ->Represents an error with a value of type E
    // }

    let number = -4.0;
    let square_root = cal_square_root(number);

    match square_root {
        Some(value) => println!("the square root of {} is {}",number, value),
        None => println!("The square root of {} is not real number.", number),
    }

    //Result

    let a = 10.0;
    let b = 0.0;
    let division = divide(a, b);

    match division {
        Err(err_msg) => println!("Error: {}",err_msg),
        Ok(value) => println!("Division of {} by {} is {}", a, b, value),
    }
}

fn cal_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a:f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a/b)
    }
}