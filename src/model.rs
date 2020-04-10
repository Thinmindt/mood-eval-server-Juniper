use chrono::{NaiveDate};
use std::collections::HashMap;
use super::schema_diesel::day_data;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(GraphQLEnum)]
pub enum MoodValue {
    Terrible,
    Bad,
    Off,
    Tired,
    Normal,
    Good,
    Marvelous,
}

#[derive(juniper::GraphQLObject, diesel::Queryable)]
#[graphql(Context = "Database")]
pub struct DayData {
    pub id: i32,
    pub day: NaiveDate,
    pub mood: String,
}

#[derive(Insertable)]
#[table_name="day_data"]
pub struct NewDayData<'a> {
    pub day: &'a NaiveDate,
    pub mood_value: &'a str,
}

// impl DayData {
//     pub fn new(
//         id: &str,
//         year: &str,
//         month: &str,
//         day: NaiveDate,
//         mood: MoodValue
//     ) -> DayData {
//         DayData {
//             id: id.to_owned(),
//             year: year.to_owned(),
//             month: month.to_owned(),
//             day: day,
//             mood: mood,
//         }
//     }
// }

pub struct Database {
    days: HashMap<String, DayData>,
}

impl Database {
    //postgress database on port 5432
    // pub fn new() -> Database {
    //     let mut days = HashMap::new();

    //     days.insert(
    //         "3000".to_owned(),
    //         DayData::new(
    //             "3000",
    //             "2020",
    //             "4",
    //             NaiveDate::from_ymd(2020, 4, 5),
    //             MoodValue::Good,
    //         ),
    //     );

    //     Database {
    //         days: days,
    //     }
    // }

    pub fn create_day_data<'a>(&self, conn: &PgConnection, day: &'a NaiveDate, mood: &'a str) -> DayData {

        let new_day = NewDayData {
            day: day,
            mood_value: mood,
        };

        diesel::insert_into(day_data::table)
            .values(&new_day)
            .get_result(conn)
            .expect("Error saving new day data")
    }

    pub fn get_day(&self, id:&str) -> Option<&DayData> {
        self.days.get(id)
    }
}