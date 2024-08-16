pub fn box_() {
    let mut pointer = Box::new(5);

    println!("pointer is {}", pointer);
    *pointer = 10;
    println!("defrefrece of pointer {}", pointer);

    let my_trait_obj: Box<dyn MakeNoise> = Box::new(Bird {
        name : "Crow".to_string(),
        color: "black".to_string(),
    });

    my_trait_obj.talk();

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();

    speakers.push(Box::new(Bird {
        name: "Parrot".to_string(),
        color: "Green".to_string(),
    }));

    speakers.push(Box::new(Dog {
        name: "Tommy".to_string(),
        breed: "Desi".to_string(),
    }));

    for speaker in speakers {
        speaker.talk();
    }
}

trait MakeNoise {
    fn talk(&self);
}

struct Bird {
    name: String,
    color: String,
}

struct Dog {
    name: String,
    breed: String,
}

impl MakeNoise for Bird {
    fn talk(&self){
        println!("That bird {} of color {} is talking name", self.name, self.color);
    }
}

impl MakeNoise for Dog {
    fn talk(&self){
        println!("That Dog {} of breed {} is talking name", self.name, self.breed);
    }
}