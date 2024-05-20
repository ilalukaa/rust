use serde::{Serialize, Deserialize};
use crate::state::Hotel;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    // Hotels
    CreateHotel { name: String, owner_name: String, location: String },
    UpdateHotel { id: u64, new_name: String, new_location: String },
    DeleteHotel { id: u64 },

    // Services
    AddService { name: String, hotel_id: u64, description: Option<String> },
    UpdateService {id: u64, new_name: String, hotel_id: u64, new_description: Option<String> },
    
    // Ratings
    AddRating { hotel_id: u64, service_id: u64, score: u8, comment: Option<String> },
    UpdateRating { hotel_id: u64, service_id: u64, new_score: u8, new_comment: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetHotel { id: u64},
    GetService { id: u64},
    GetAllHotels { filter: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllHotelsResponse {
    pub hotels: Vec<Hotel>,
}