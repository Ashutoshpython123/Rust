
pub fn data_type(){
   println!("------data type-----");
   let name = "Ashutosh"; //string
   let age = 27.5; //float
   let is_married = false; //bool

   println!("name: {}", name);
   println!("age : {}", age);
   println!("married : {}", is_married);
   println!("**********Interger**********");

   let result = 10;    // i32 by default
   let sum:i32 = 5-15;
   let mark:isize = 10;
   let count:usize = 30;
   println!("result value is {}",result);
   println!("sum is {}",sum);
   println!("mark is {} and count is {}",mark,count);


   println!("overflow : 0 to 255 only allowed for u8");
   let a:u8 = 255;

   // let weight:u8 = 256;   //overflow value is 0
   // let height:u8 = 257;   //overflow value is 1
   println!("a is {} ",a);

   println!("**********Float**********");

   let result_f = 10.00;        //f64 by default
   let interest:f32 = 8.35;     //single precision
   let cost:f64 = 15000.600;  //double precision
   
   println!("result_f value is {}",result_f);
   println!("interest is {}",interest);
   println!("cost is {}",cost);

   println!("Automatic type casting is not allowed in Rust.");

   let float_with_separator = 11_000.555_001;
   println!("float value {}",float_with_separator);
   
   let int_with_separator = 50_000;
   println!("int value {}",int_with_separator);

   println!("**********Boolean**********");


   let isfun:bool = true;
   println!("Is Rust Programming Fun ? {}",isfun);

   println!("**********Character**********");


   let special_character = '@'; //default
   let alphabet:char = 'A';
   let emoji:char = '😁';
   
   println!("special character is {}",special_character);
   println!("alphabet is {}",alphabet);
   println!("emoji is {}",emoji);

}