#[macro_use]
extern crate rocket;
mod menu;
mod schema;
use menu::Meal;
use menu::Menu;
use menu::StudentMeal;
use rocket::form::Form;
use rocket::serde::json::Json;
#[delete("/all")]
fn delete_all() -> String {
    Menu::delete_all();
    "Everything deleted successfully".to_string()
}
#[delete("/", data = "<student_form>")]
fn remove_student(student_form: Form<StudentMeal>) -> String {
    let student = student_form.into_inner();
    Menu::remove_student(&student);
    "Meal removed successfully".to_string()
}
#[post("/", data = "<meal_form>")]
fn add_meal(meal_form: Form<Meal>) -> String {
    let meal = meal_form.into_inner();
    if meal.empty() {
        "Something is empty".to_string()
    } else {
        Menu::insert(meal);
        "Menu Entered".to_string()
    }
}
#[post("/", data = "<student_form>")]
fn add_student(student_form: Form<StudentMeal>) -> String {
    let student = student_form.into_inner();
    Menu::add_student(&student);
    "Meal added successfully".to_string()
}
#[get("/admin")]
fn admin_menu() -> Json<Vec<Menu>> {
    Json(Menu::all().expect("Could not retrieve the menu"))
}
#[get("/student", data = "<student_form>")]
fn student_menu(student_form: Form<StudentMeal>) -> Json<Vec<Menu>> {
    let student = student_form.into_inner();
    Json(Menu::menu_for_student(student).expect("Could not retrieve the menu"))
}
#[get("/")]
fn index() -> String {
    "Hello".to_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/menu",
        routes![
            add_student,
            remove_student,
            add_meal,
            delete_all,
            admin_menu,
            student_menu
        ],
    )
}
//TODO
//index function. have it link to front end
//get meals for a student
//post meals for student
//get meal list in format for admin
//get menu from admin
