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


    //Combined of Option and Result
    let base = get_from_arr("base".to_string());
    let height = get_from_arr("height".to_string());
    let area = cal_triangle_area(base, height);
    match area {
        Ok(value) => println!("area of triangle: {}", value),
        Err(err) => println!("Error : {}", err),
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

fn get_from_arr(key: String) -> Option<f64> {
    let db = vec![("base", Some(10.0)), ("height", Some(6.0))];
    for (k, v) in db {
        if k == key {
            return v;
        }
    }
    None
}


fn cal_triangle_area(base : Option<f64>, height : Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b<=0.0 || h <=0.0 {
                Err("both base and height must be positive number".to_string())
            } else {
                Ok(0.5*b*h)
            }
        },
        (None, _) => Err("Some thing wrong with height".to_string()),
        (_, None) => Err("Some thing wrong with base".to_string()),
    }
}
