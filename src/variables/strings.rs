pub fn main() {
    demonstrate_string_types();
}

// Overview of the most common string types in Rust
// PL: Przegląd najczęściej używanych typów łańcuchów znaków w Rust
fn demonstrate_string_types() {
    // String Literal (&str) - immutable reference to text stored in the program binary
    // PL: Literał łańcuchowy (&str) - niezmienna referencja do tekstu w pamięci programu
    let string_literal: &str = "Hello, world!";
    println!("String Literal (&str): {}", string_literal);

    // Owned String (String) - heap-allocated, can be modified
    // PL: Posiadany łańcuch (String) - alokowany na stercie, można go modyfikować
    let mut owned_string: String = String::from("Hello, owned world!");
    owned_string.push_str(" More text.");
    println!("Owned String (String): {}", owned_string);

    // Raw String Literal - no escaping of special characters
    // PL: Surowy literał łańcuchowy - bez escapowania znaków specjalnych
    let raw_string_literal: &str = r#"Hello, "raw" world!"#;
    println!("Raw String Literal: {}", raw_string_literal);

    // Byte String Literal (&[u8])
    // PL: Bajtowy literał łańcuchowy (&[u8])
    let byte_string_literal: &[u8] = b"Hello, byte world!";
    println!("Byte String Literal (&[u8]): {:?}", byte_string_literal);

    // Byte String (Vec<u8>) - ASCII values of "Hello, byte world!"
    // PL: Łańcuch bajtów (Vec<u8>) - wartości ASCII dla "Hello, byte world!"
    let byte_string: Vec<u8> = vec![
        72, 101, 108, 108, 111, 44, 32, 98, 121, 116, 101, 32, 119, 111, 114, 108, 100, 33,
    ];
    println!("Byte String (Vec<u8>): {:?}", byte_string);

    // OsString - string in the operating system's native format
    // PL: OsString - łańcuch w formacie natywnym dla systemu operacyjnego
    use std::ffi::OsString;
    let os_string = OsString::from("Hello, OS world!");
    println!("OsString: {:?}", os_string);

    // CString - null-terminated string, compatible with C
    // PL: CString - łańcuch zakończony zerem, zgodny z językiem C
    use std::ffi::CString;
    let c_string = CString::new("Hello, C world!").expect("CString::new failed");
    println!("CString: {:?}", c_string);

    // Rc<str> - shared (reference-counted), immutable string
    // PL: Rc<str> - współdzielony (zliczanie referencji), niezmienny łańcuch
    use std::rc::Rc;
    let rc_str: Rc<str> = Rc::from("Hello, Rc<str> world!");
    println!("Rc<str>: {:?}", rc_str);
}
