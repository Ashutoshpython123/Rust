pub fn tuple_(){
    println!("tuple");
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}",tuple);

    struct Point3D(f32, f32, f32);
    let point = Point3D(10.0, 20.0, 30.0);
    println!("x={}, y={}, z={}", point.0, point.1, point.2)
}