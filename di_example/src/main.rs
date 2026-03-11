mod examples;

fn main() {
    println!("Delegate run");
    examples::delegate_di::run();
    println!();

    println!("No DI run");
    examples::no_di::run();
    println!();

    println!("DI no init run");
    examples::di_no_init::run();
    println!();

    println!("Struct enumerator run");
    examples::struct_enumerator::run();
    println!();

    println!("DI with init");
    examples::di_init::run();
    println!();

    println!("DI Polymorphism");
    examples::di_polymorphism::run();
    println!();
}
