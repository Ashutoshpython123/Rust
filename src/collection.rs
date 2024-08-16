use std::collections::HashMap;

pub fn collection() {
    //Vecotor
    let mut numbers = vec![1,2,3,4];
    let mut names: Vec<String> = Vec::new();

    names.push(String::from("Ashutosh"));
    names.push(String::from("Maurya"));

    println!("first name : {}, Second name : {}", names[0], names[1]);
    names.pop();

    // for number in numbers {
    //     println!("number is : {}", number);
    // }

    let slice = &numbers[1..3];
    for num in slice {
        println!("number is : {}", num);
    }

    //String
    let mut mystring = String::from("Ashutosh");
    let mut sencond_string = "Maurya".to_string();

    mystring.push_str(" Maurya");
    println!("my string : {}", mystring);

    for chr in sencond_string.chars() {
        println!("char : {}", chr);
    }

    for byte in sencond_string.bytes(){
        println!("bytes : {}", byte);
    }


    //Hashmap

    let mut score = HashMap::new();
    score.insert(String::from("John"), 10);
    let doe_score = score.insert(String::from("Doe"), 20);

    let john_score = score.get(&String::from("John"));

    println!("john score: {:?}", john_score);
    println!("scores : {:?}", score);

    for (key, value) in &score {
        println!("key: {}, value: {}", key, value);
    }

    score.remove(&String::from("John"));
    println!("scores : {:?}", score);

}