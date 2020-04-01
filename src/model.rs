use std::collections::HashMap;

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

#[derive(juniper::GraphQLObject)]
#[graphql(Context = "Database")]
pub struct DayData {
    id: String,
    year: String,
    month: String,
    day: String,
    mood: MoodValue,
}

impl DayData {
    pub fn new(
        id: &str,
        year: &str,
        month: &str,
        day: &str,
        mood: MoodValue
    ) -> DayData {
        DayData {
            id: id.to_owned(),
            year: year.to_owned(),
            month: month.to_owned(),
            day: day.to_owned(),
            mood: mood,
        }
    }
}

pub struct Database {
    days: HashMap<String, DayData>,
}

impl Database {
    pub fn new() -> Database {
        let mut days = HashMap::new();

        days.insert(
            "3000".to_owned(),
            DayData::new(
                "3000",
                "2020",
                "4",
                "1",
                MoodValue::Good,
            ),
        );

        Database {
            days: days,
        }
    }

    pub fn get_day(&self, id:&str) -> Option<&DayData> {
        self.days.get(id)
    }
}