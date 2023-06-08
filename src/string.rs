pub fn hello() {
    let mut hello = String::from("Hey");
    println!("{}",hello);
    println!("length of above{}",hello.len());
    // push charcater only one char
    // '' needed
    hello.push('d');
    println!("{}",hello);
    // push string
    // "" needed
    hello.push_str(" holy");
    println!("{}",hello);
    // capacity in bytes
    println!("hello {}",hello.capacity());
    println!("random word: jjj {}",
             String::from(
                 "jjj"
             ).capacity()
    );
    println!("random word: kkk {}",
             String::from(
                 "kkk"
             ).capacity()
    );
    println!("random word: jj j {}",
             String::from(
                 "jj j"
             ).capacity()
    );
    println!("jj j is empty {}",
             String::from(
                 "jj j"
             ).is_empty()
    );
    println!("is empty {}",
             String::from(
                 ""
             ).is_empty()
    );
    println!(" is empty {}",
             String::from(
                 " "
             ).is_empty()
    );
    println!(" is contain {}",
             String::from(
                 " "
             ).contains("")
    );
    println!(" is contain  {}",
             String::from(
                 " "
             ).contains(" ")
    );
    println!(" is contain        {}",
             String::from(
                 " "
             ).contains("        ")
    );
    println!("replace {}'s {} with {} = {}",
             String::from(
                 "Hall of Hog"
             ),"Hog","Fame",
             String::from("Hall of Hog").replace(
                 "Hog","Fame"
             )
    );
    // for loop
    for k in "a sd as d".split_whitespace() {
        println!("{}",k);
    }
}
