// Declare the modules (each lives in its own directory under src/)
// PL: Deklaracja modułów (każdy znajduje się we własnym katalogu w src/)
mod hello;
mod variables;
mod concurrency;
mod binding_and_mutability;

// Program entry point - uncomment the module you want to run
// PL: Punkt wejścia programu - odkomentuj moduł, który chcesz uruchomić
fn main() {
    // hello::main();
    // variables::main();
    // concurrency::main();
    binding_and_mutability::main();
}
