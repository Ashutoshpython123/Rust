pub fn enum_(){
    let msg = Message::Write(String::from("hello Rust!"));
    // process_message(msg);
    msg.call();


    let my_text = Message::Write("jadu".to_string());
    if let Message::Write(text) = my_text {
        println!("write msg :{}", text);
    }else{
        println!("test is not hello Rust!");
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
        }
    }
}

fn process_message(msg:Message){
    match msg{
        Message::Quit => {
            println!("the quit varient has no data");
        }
        Message::Move{x, y} => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
        Message::Write(text)=> {
            println!("write text : {}",text);
        }
        Message::ChangeColor(x,y,z) => {
            println!("change color x: {}, y: {}, z: {}",x,y,z);
        }
    }
}