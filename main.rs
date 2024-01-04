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


use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2024-01-04"));
}



