pub fn array(){
    println!("------array--------");
    let arr = [1,2,3];
    println!("array : {:?}, length : {}", arr, arr.len());
    let arr1:[i32;3] = [1,2,3];
    println!("array1 : {:?}, length : {}", arr1, arr1.len());

    for item in arr1.iter(){
        println!("item : {}", item);
    }

}