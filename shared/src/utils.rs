use uuid::Uuid;
use time::OffsetDateTime;

pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn current_timestamp() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}