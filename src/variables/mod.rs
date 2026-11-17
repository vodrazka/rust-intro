// Submodules - each in its own file in this directory
// PL: Podmoduły - każdy we własnym pliku w tym katalogu
mod tuples;
mod strings;
mod integers;

pub fn main() {
    println!("Module \"variables\" loaded");
    // Uncomment the submodule you want to run
    // PL: Odkomentuj podmoduł, który chcesz uruchomić
    strings::main();
    //integers::main();
    //tuples::main();
    println!("Module \"variables\" has everything commented out");
}
