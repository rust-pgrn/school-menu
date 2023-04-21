mod menu;
mod schema;
use diesel::pg::data_types::PgMoney;
use menu::Meal;
use menu::Menu;
fn main() {
    let meal = Meal {
        date: "12/12/2023".to_string(),
        meal: "Kofta".to_string(),
        students: vec![None],
        price: PgMoney(66),
    };
    //Menu::insert(meal);
    //println!("{:?}", Menu::all().expect("Could not load menu"));
    let meal = Meal {
        date: "12/14/2023".to_string(),
        meal: "Molokhia".to_string(),
        students: vec![None],
        price: PgMoney(50),
    };
    //Menu::insert(meal);
    //println!("{:#?}", Menu::all().expect("Could not load menu"));
    println!("{:?}", Menu::id(2));
    Menu::add_student("Shams".to_string(), 1, false);
    //println!("\n\n\n\n\n");
    println!("{:#?}", Menu::all().expect("Could not load menu"));
}
