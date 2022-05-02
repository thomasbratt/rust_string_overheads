use std::mem::size_of_val;

const DATA: &str = "Hello World";

pub struct Holder<'a> {
    field_box_of_str: Box<&'a str>,
    field_box_of_string: Box<String>,
}

// Prints size of various string related types.
// Comments assume 64 bit for usize and pointer.
pub fn main() {
    print_raw_data();
    print_str_ref();
    print_box_of_str_ref();
    print_string();
    print_box_of_string();
    print_holder();
}

fn print_raw_data() {
    println!();
    let value = DATA;
    println!("str '{}' (internally: byte per character)", value);
    dbg!(size_of_val::<str>(value));
}

fn print_str_ref() {
    println!();
    let value = &DATA;
    println!("&str (internally: pointer+length)");
    dbg!(size_of_val::<&str>(value));
}

fn print_box_of_str_ref() {
    println!();
    println!("Box<&str> (internally: pointer+&str)");
    let value: Box<&str> = Box::new(DATA);
    dbg!(size_of_val::<Box<&str>>(&value));
    println!(
        "Boxed<&str> overhead is {}+{}={} bytes",
        size_of_val::<Box<&str>>(&value),
        size_of_val::<&str>(&DATA),
        size_of_val::<Box<&str>>(&value) + size_of_val::<&str>(&DATA)
    );
}

fn print_string() {
    println!();
    println!("String (internally: pointer+length+capacity)");
    let value = DATA.to_string();
    dbg!(size_of_val::<String>(&value));
}

fn print_box_of_string() {
    println!();
    println!("Box<String> (internally: pointer+String)");
    let value: Box<String> = Box::new(DATA.to_string());
    dbg!(size_of_val::<Box<String>>(&value));
    println!(
        "Box<String> overhead is {}+{}={} bytes",
        size_of_val::<Box<String>>(&value),
        size_of_val::<String>(&DATA.to_string()),
        size_of_val::<Box<String>>(&value) + size_of_val::<String>(&DATA.to_string())
    );
}

fn print_holder() {
    println!();
    println!("Holder (internally: pointer+pointer)");
    let value = Holder {
        field_box_of_str: Box::new(&DATA),
        field_box_of_string: Box::new(DATA.to_string()),
    };
    dbg!(size_of_val::<Holder>(&value));
    println!(
        "Box<Holder> overhead is {} bytes",
        size_of_val::<Holder>(&value),
    );
}
