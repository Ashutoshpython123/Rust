pub fn loops(){
    println!("------Definite Loop-----------");
    println!("forLoop : ");

    for x in 0..5{
        println!("x : {}",x);
    }

    println!("------Indefinite Loop-----------");

    let mut i = 0;
    while i < 5{
        i += 1;
        println!("i : {}", i);
    }


    let mut x = 0;
    loop {
      x+=1;
      println!("x={}",x);

      if x==5 {
         break;
      }
    }
}