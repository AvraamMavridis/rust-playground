struct Customer<'a, 'b> {
    name: &'a String,
    email: &'b String,
    active: bool,
}

fn build_customer<'a, 'b>(name: &'a String, email: &'a String, active: bool) -> Customer<'a, 'b> {
    Customer<'a, 'b> {
        name: name,
        active: active,
        email: email,
    }
}

fn main() {
    const name : String = String::from("sda");
    const email : String = String::from("sda");
    build_customer(&name, &email, true);
    println!("Hello, world!");
}
