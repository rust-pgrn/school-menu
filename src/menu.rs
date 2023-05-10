use super::schema::menu;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;
use dotenvy::dotenv;
use std::env;

#[derive(Queryable, Debug, Clone)]
pub struct Menu {
    pub id: i32,
    pub date: String,
    pub meal: String,
    pub students: Vec<Option<String>>,
    pub price: i32,
}

#[derive(Debug, FromForm)]
pub struct StudentMeal {
    name: String,
    meal_id: i32,
    substitute: bool,
}
#[derive(Insertable, Debug, FromForm)]
#[diesel(table_name = menu)]
pub struct Meal {
    pub date: String,
    pub meal: String,
    pub students: Vec<Option<String>>,
    pub price: i32,
}
impl Meal {
    pub fn empty(&self) -> bool {
        self.meal.is_empty() && self.date.is_empty() && self.price.eq(&0)
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl Menu {
    //Will be async in future when we have async establish_connection
    pub fn all() -> QueryResult<Vec<Menu>> {
        let mut connection = &mut establish_connection();
        menu::table.load::<Menu>(connection)
    }
    pub fn id(meal_id: i32) -> Menu {
        let mut connection = &mut establish_connection();
        menu::table
            .find(meal_id)
            .first(connection)
            .expect("Error loading meal")
    }
    pub fn insert(meal: Meal) -> Menu {
        let mut connection = &mut establish_connection();
        diesel::insert_into(menu::table)
            .values(meal)
            .get_result(connection)
            .expect("Error saving meal")
    }
    pub fn insert_list(meals: Vec<Meal>) -> Vec<Menu> {
        let mut connection = &mut establish_connection();
        diesel::insert_into(menu::table)
            .values(meals)
            .get_results(connection)
            .expect("Error saving meal")
    }
    pub fn add_student(student: &StudentMeal) {
        //students.push(Some(student));
        let menu = Menu::id(student.meal_id);
        let mut students = menu.students;
        students.push(Some(format!("{} {}", student.name, student.substitute)));
        let mut connection = &mut establish_connection();
        diesel::update(menu::table.find(student.meal_id))
            .set(menu::students.eq(students))
            .execute(connection)
            .unwrap();
    }
}
