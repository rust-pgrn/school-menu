#[macro_use]
extern crate rocket;
mod menu;
mod schema;
use menu::StudentMeal;
use rocket::form::Form;
use std::fs;

use diesel::pg::data_types::PgMoney;
use menu::Meal;
use menu::Menu;
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
            /*remove_student, menu, admin_menu,*/ add_meal
        ],
    )
}
//TODO
//index function. have it link to front end
//get meals for a student
//post meals for student
//get meal list in format for admin
//get menu from admin
