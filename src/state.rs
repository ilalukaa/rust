use cosmwasm_std::{Addr};
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Map, Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hotel {
    pub id: u64,
    pub name: String,
    pub owner: Addr,
    pub owner_name: String,
    pub location: String,
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Service {
    pub id: u64,
    pub name: String,
    pub hotel_id: u64,
    pub ratings: Vec<Rating>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Rating {
    pub user: Addr,
    pub id: (u64, Addr),
    pub hotel_id: u64,
    pub service_id: u64,
    pub score: u8, // Rating out of 5 for simplicity
    pub comment: Option<String>,
}

// State
pub const HOTELS: Map<u64, Hotel> = Map::new("hotels");
pub const SERVICES: Map<u64, Service> = Map::new("services");
pub const RATINGS: Map<(u64, Addr), Rating> = Map::new("ratings");

pub const HOTEL_COUNT: Item<u64> = Item::new("hotel_count");
pub const SERVICE_COUNT: Item<u64> = Item::new("service_count");
