pub fn string(){
   println!("------STRING-----");

   println!("String Literal(&str) :");
   
   let name:&str = "Ashutosh";
   println!("name is {}", name);
   let location:&'static str = "mohali";
   println!("location static {}", location);

   println!("String Object(String) : ");

   let empty_string = String::new();
   println!("length is {}",empty_string.len());

   let name_string = String::from("Ashutosh");
   println!("length is {}",name_string.len());
}