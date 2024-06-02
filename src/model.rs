use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub phone_number: String,
    pub night: String,
    pub zone: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserTicketOrder {
    pub user_info: UserInfo,
    pub seats: Vec<Seat>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Seat {
    pub seat_number: String,
}
