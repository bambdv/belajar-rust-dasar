fn main() {
    println!("Hello, world!");
}

#[test] // ! istilahnya : annotation

fn test_hello_word() {
    println!("Hellao")
}

#[test]
fn variable() {
    let name: &str = "Ibrohim Sairony";
    println!("Hello {}", name)
}

#[test]
fn mutable() {
    let mut name = "Sairony";
    println!("Hello {}", name);
    name = "Ibrohim Sairony";
    println!("Hello {}", name)
}

// #[test]
// fn static_typing() { // ! type data tidak bisa diubah
//     let mut name = "Sairony";
//     name = 10;
//     println!("Hello {}", name)
// }

#[test]
fn shadowing() {
    let name = "Sairony";
    println!("Hello {}", name);

    // ! variable seblumnya akan tertutupi
    let name = 10;
    println!("Hello {}", name)
}

#[test]
fn type_data() {
    // ! secara umum type data ada 2 scalar dan compound
    // * scalar yang datanya satu.
    // * compound seperti array dan tuple
    let age: i32 = 20;
    println!("Age {}", age)
}

#[test]
fn explicit_type_data() {
    let age: i32 = 20;
    println!("Age {}", age)
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("Age {}", a);

    let b: f32 = 10.4;
    println!("float {}", b)
}

#[test]
fn number_confersion() {
    let a: i8 = 10;
    let b: i32 = a as i32;
    println!("{}", b);
    let c: i64 = b as i64;
    println!("{}", c);

    // ! overflow confersion
    let d: i64 = 100000000000000;
    let f: i8 = d as i8;
    println!("{}", f);
}

#[test]
fn number_operator() {
    let a = 10;
    let b = 30;
    let c = a % b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let f = a * b;
    println!("{}", f);
    let g = a + b;
    println!("{}", g);
    let h = a - b;
    println!("{}", h);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);

    a *= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b = false;
    println!("{} , {}", a, b)
}

#[test]
fn comparison() {
    // <  >  <=  >=  ==  !=
    let result = 30 > 110;
    println!("{}", result);
}

#[test]
fn operator_boolean() {
    // && || !
    let result = true || false;
    println!("{}", result);
}

#[test]
fn char_type() {
    // ! pakai petik ''
    let char1 = 'a';
    let char2 = 'b';
    println!("{}, {}", char1, char2);
}

#[test]
fn tuple() {
    let data = (10, 10.5, true); //  * bisa beda tipe data
    println!("{:?}", data);
    // * {:?}  artinya : debug information

    // let a = data.0;
    // let b = data.1;
    // let b = data.2;

    let (a, b, c) = data;
    println!("{}, {}, {}", a, b, c);

    let (f, g, _) = data; // * tanda _ untuk mencuekin
    println!("{}, {}", f, g);
}

#[test]
fn mutable_tuple() {
    let mut data = (10, 10.5, true);
    println!("{:?}", data);

    data.0 = 40;
    data.1 = 20.4;
    data.2 = false;
    println!("{:?}", data);
}

// ! Pembahasan unit. Unit adalah tuple kosong ()
fn unit() {
    // return value tuple kosong
    println!("Hello!")
}
#[test]
fn test_unit() {
    let result: () = unit(); // result adalah tuple kosong / unit ()
    println!("{:?}", result);

    let test_unit: () = (); // langsung membuat unit
    println!("{:?}", test_unit);

    //unit jarang digunakan
}
#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5]; //  * fix tipe data
    println!("{:?}", array);

    let a = array[0]; // * pakai [] kalau tuple pakai .
    let b = array[1];
    println!("{}, {}", a, b);
}

#[test]
fn mutable_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);
}

#[test]
fn length_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let panjang: usize = array.len();
    println!("{}", panjang);
}

#[test]
fn two_dimensional_array() {
    let matrix = [[1, 2], [1, 2]]; // bisa bertambah array didalam array lagi

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
}

const MINIMUM: i32 = 0;
#[test]
fn constant() {
    const MAXIMUM: i32 = 100;
    println!("{} {}", MAXIMUM, MINIMUM);
}

#[test]
fn variable_scope() {
    println!("{}", MINIMUM); // global_scope

    let ibrohim = 0; // variable_scope

    {
        // inner_scope
        println!("{}", ibrohim); // success
        let _sairony = 1;
    }
    // println!("{}", sairony); // * error - outter_scope
}

#[test]
fn stack_heap() {
    // Manajement Memory ada 2 : stack dan heap
    function_a();
    function_b()
}

fn function_a() {
    let a = 10;
    let b = String::from("Ibrohim");
    println!("{}, {}", a, b)
}

fn function_b() {
    let a = 10;
    let b = String::from("Sairony");
    println!("{}, {}", a, b);
}

#[test]
fn string_slice() {
    // string_slice beda dengan string biasa
    // kode type_data : &str
    let name = "   Ibrohim Sairony    ";
    let trim = name.trim();
    println!("{}", name);
    println!("{}", trim);

    let mut _username = "ibrohim"; // fixed-size.  gk bisa diapa-apakan. selalu ada di memory.
    _username = "Sairony"; // data "ibrohim" gk hilang. cuma nama variable diganti.
    println!("{}", _username);

    // number juga sama seperti string_slice : fixed-size
    let mut _value = 10;
    _value = 20 // ! aku baru tau perilaku seprti ini
}

#[test]
fn string() {
    let mut name = String::from("Ibrohim");
    println!("{}", name);

    name.push_str(" Sairony");
    println!("{}", name);

    let saad = name.replace("Ibrohim", "Saad");
    println!("{}", saad);
}

#[test]
fn data_copy() {
    // untuk data di stack (fixed-size) implement copy, jadi variable / owner lama bisa diakses
    let a = 10;
    let b = a;
    println!("{}, {}", a, b);
}

#[test]
fn ownership_movement() {
    // untuk data di heap (data-bisa-berkembang) itu bukan copy melainkan move jadi owner lama sudah hilang
    let name1 = String::from("Ibrohim");
    println!("{}", name1);

    let name2 = name1; //ownership pindah ke name2
    println!("{}", name2);
    // println!("{}", name1); // ! error karena ownership sudah berpindah
}

#[test]
fn clone() {
    // solusi untuk copy data heap. bukan movement_ownership
    let name1 = String::from("Ibrohim");
    let name2 = name1.clone();
    println!("{}, {}", name1, name2)
}
