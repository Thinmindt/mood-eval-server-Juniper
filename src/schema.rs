use crate::model::{Database, DayData};
use juniper::{Context};

impl Context for Database {}

pub struct Query;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
/// The root query object of the schema
impl Query {
    #[graphql(arguments(id(description = "id of the day")))]
    fn day(database: &Database, id: String) -> Option<&DayData> {
        database.get_day(&id)
    }
}
