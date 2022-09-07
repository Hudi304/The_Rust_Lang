use inquire::Select;

#[allow(dead_code, unused)]
pub fn inquire() {
    let string_literal_array = ["START", "STOP"];

    let string_array = string_literal_array.map(|x| String::from(x));

    menu(&string_array);
}
#[allow(dead_code, unused)]
fn menu(items: &[String]) -> String {
    return Select::new("MENU", items.to_vec()).prompt().unwrap();
}
