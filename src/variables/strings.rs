pub fn main() {
    demonstrate_string_types();
}

fn demonstrate_string_types() {
    // String Literal (&str)
    let string_literal: &str = "Hello, world!";
    println!("String Literal (&str): {}", string_literal);

    // Owned String (String)
    let mut owned_string: String = String::from("Hello, owned world!");
    owned_string.push_str(" More text.");
    println!("Owned String (String): {}", owned_string);

    // Raw String Literal
    let raw_string_literal: &str = r#"Hello, "raw" world!"#;
    println!("Raw String Literal: {}", raw_string_literal);

    // Byte String Literal (&[u8])
    let byte_string_literal: &[u8] = b"Hello, byte world!";
    println!("Byte String Literal (&[u8]): {:?}", byte_string_literal);

    // Byte String (Vec<u8>)
    let byte_string: Vec<u8> = vec![
        72, 101, 108, 108, 111, 44, 32, 98, 121, 116, 101, 32, 119, 111, 114, 108, 100, 33,
    ];
    println!("Byte String (Vec<u8>): {:?}", byte_string);

    // OsString
    use std::ffi::OsString;
    let os_string = OsString::from("Hello, OS world!");
    println!("OsString: {:?}", os_string);

    // CString
    use std::ffi::CString;
    let c_string = CString::new("Hello, C world!").expect("CString::new failed");
    println!("CString: {:?}", c_string);

    // Rc<str>
    use std::rc::Rc;
    let rc_str: Rc<str> = Rc::from("Hello, Rc<str> world!");
    println!("Rc<str>: {:?}", rc_str);
}
