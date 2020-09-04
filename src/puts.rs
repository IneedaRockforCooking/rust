pub fn run() {
    println!("asas");
}

pub fn joke() {
    println!("only options: {}",2);
}

pub fn ggg() {
    // Positional Arguments
    println!("{0} {1} {1} {0}","1",2);
    // Name Arguments
    println!("{j} {o}",j=1,o="123");
    // Placeholder Traits
    println!("binary {:\
    b} hex {:\
    x} octal {:o\
    }",10,
             10,10);
    // Placeholder for Debug Trait
    println!("{:?}",(12,true,"he"));
    // Basic Math
    println!("10 + 10 = {}",21);
}