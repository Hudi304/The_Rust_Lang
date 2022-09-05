use std::string;

use inquire::Select;

pub fn inquire() {
    let string_litteral_array = ["START", "STOP"];

    let string_array = string_litteral_array.map(|x| String::from(x));

    menu(&string_array);
}

fn menu(items: &[String]) -> String {
    return Select::new("MENU", items.to_vec()).prompt().unwrap();
}
