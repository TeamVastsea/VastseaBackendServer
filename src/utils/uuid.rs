use bson::Uuid;

pub fn generate_uuid() -> String {
    let uuid = Uuid::new().to_string().replace("-", "");
    uuid
}