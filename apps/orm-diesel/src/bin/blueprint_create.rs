use chrono::{Duration, Utc};
use diesel::{RunQueryDsl, SelectableHelper};

use orm_diesel::models::{Blueprint, BlueprintView, NewBlueprint};

fn get_cst_time() -> chrono::NaiveDateTime {
    let now_utc: chrono::DateTime<Utc> = Utc::now();
    let now_naive: chrono::NaiveDateTime = now_utc.naive_local();
    let now_cst: chrono::NaiveDateTime = now_naive + Duration::hours(8);
    now_cst
}
pub fn create_blueprint() -> Blueprint {
    use orm_diesel::schema::t_blueprint;
    use orm_diesel::establish_connection;
    let new_post = NewBlueprint {
        name: "test",
        description: "",
        user_id: "1",
        config: "hello world",
        create_time: &get_cst_time(),
        update_time: &get_cst_time(),
    };

    diesel::insert_into(t_blueprint::table)
        .values(&new_post)
        .returning(Blueprint::as_returning())
        .get_result(&mut establish_connection())
        .expect("Error saving new post")
}

fn main() {
    let b =  create_blueprint();
    let blueprint_view = BlueprintView::from(&b);
    let value = serde_json::to_string_pretty(&blueprint_view).unwrap();
    println!("{}", value);
    println!("{:?}",b)
}