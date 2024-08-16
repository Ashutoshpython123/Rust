pub fn traits() {
    let dog = Dog {
        name: "tony".to_string(),
    };
    dog.speak();

    let cow = Cow {
        name: "Gau mata".to_string(),
    };

    cow.speak();

    let original_string = String::from("this is original.");
    let display_string = original_string.display();
    println!("display string: {}", display_string);

    animal_speak(&dog);
    animal_speak(&cow);
}

trait Speak {
    fn speak(&self);
}

struct Dog{
    name : String,
}

struct Cow {
    name: String,
}

impl Speak for Dog {
    fn speak(&self){
        println!("{} says woof.", self.name);
    }
}

impl Speak for Cow {
    fn speak(&self){
        println!("{} says mooo.", self.name);
    }
}

trait Display {
    fn display(&self) -> String;
}

impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}

fn animal_speak<T: Speak>(animal: &T) {
    animal.speak();
}