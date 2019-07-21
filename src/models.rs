// These fields must be defined in the order they are found
// in the src/schema.rs (generated based on migration), 
// otherwise Diesel will return them in a
// unexpected order.
#[derive(Queryable)]
pub struct Request {
    pub id: i32,
    pub name: String,
    pub uri: String,
    pub response_status_code: String,
    pub success: bool,
}
