pub fn main_fn(){
    let x = 5;
    let y = 10;
    let sum = add_number(x,y);
    println!("add two number {x} + {y} = {sum}");
}

fn add_number(x:i32, y:i32) -> i32 {
    let result = x + y;
    return result;
}