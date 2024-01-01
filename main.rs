fn main() {
    let x= 10;
    let y = x;
    // string value is copy
    println ("{}", x)
    println ("{}", y)
}

fn main() {
    let x= string::from("EzSnippet"); 
    let y = x;
        // String value is not coppied in
    println ("{}", x)
    println ("{}", y)
}

fn main() {
    let x= string::from("EzSnippet"); 
    let y = x.clone();
    // to copy string value we need to use clone()
    println ("{}", x)
    println ("{}", y)
}

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

 

