fn main() {
    //Syntax for constructing and accessing
    let mut Namedd = Named {
        x: 45,
        y: 45e-10,
        z: "Named Struct".to_string(),
    };
    //Accessing
    println!("{} \n {:} \n {}", Namedd.x, Namedd.y, Namedd.z);
    //Modifying
    Namedd.x = 67;
    Namedd.z.push('I');

    let mut Tuplee = Tuple("Tuple".to_string(), 56.7, vec![1, 3, 7, 3]);
    //Accessing
    println!("{}\n {}\n {:?}", Tuplee.0, Tuplee.1, Tuplee.2);
    //Modifying
    Tuplee.0.push_str("Extra tuple");
    Tuplee.2.extend(&[12, 67, 45]);

    let Unitt = Unit;
    println!("{}", std::mem::size_of::<Unit>());
}
//Named struct
struct Named {
    x: i32,
    y: f64,
    z: String,
}

//Tuple struct
struct Tuple(String, f64, Vec<i32>);

//Unit Struct
struct Unit;
