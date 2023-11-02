#[allow(dead_code)]
pub fn prompt(s: &str) {
    let divider = "=".repeat(s.len());
    println!("\n{}", divider);
    println!("{}", s);
    println!("{}\n", divider);
}

#[allow(dead_code)]
pub fn section(s: &str) {
    println!("\n# {}\n# ...\n", s);
}

#[allow(dead_code)]
pub fn message(s: &str) {
    println!("> {}", s);
}

#[allow(dead_code)]
pub fn show_display<T>(object: &T)
where T: std::fmt::Display {
    let s = format!("{}", *object);
    for l in s.lines() { println!("> {}", l) }
}

#[allow(dead_code)]
pub fn show_debug<T>(object: &T)
where T: std::fmt::Debug {
    let s = format!("{:?}", *object);
    for l in s.lines() { println!("> {}", l) }
}

#[allow(dead_code)]
pub fn show_pretty_print<T>(object: &T)
where T: std::fmt::Debug {
    let s = format!("{:#?}", *object);
    for l in s.lines() { println!("> {}", l) }
}

#[allow(dead_code)]
pub fn show_items<T>(items: &Vec<T>)
where T: std::fmt::Display {
    let text = items.iter().map(|n| format!("{}", n)).collect::<Vec<_>>().join(", ");
    message(&text);
}
