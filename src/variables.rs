pub fn variable() {
   println!("------Variable-----");

   let mut value = 25;
   println!("value is {}", value);
   value = 10;
   println!("value is {}", value);

   let uname = "Ashutosh";
   let uname = uname.len();
   println!("name changed to integer : {}",uname);
}