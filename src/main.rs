mod string;
mod types;
mod lol;
mod puts;
mod var;

fn main() {
    puts::run();
    println!("Hello, world!");
    puts::joke();
    puts::ggg();
    var::run();
    lol::shit();
    types::run();

    let k = false;
    // Boolean
    println!("{:?}",(
        k
        ));
    // Get boolean from
    let jojo = 1000 > 100000%5;
    println!("{}",jojo);

    let smile = "\u{1f600}";

    println!("{}",smile);

    string::hello();
}
