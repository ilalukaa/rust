use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, AllHotelsResponse};
use crate::state::{Hotel, Service, Rating, HOTELS, SERVICES, RATINGS, HOTEL_COUNT, SERVICE_COUNT};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    HOTEL_COUNT.save(deps.storage, &0)?;
    SERVICE_COUNT.save(deps.storage, &0)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateHotel { name, owner_name, location } => try_create_hotel(deps, info, name, owner_name, location),
        ExecuteMsg::UpdateHotel { id, new_name, new_location } => try_update_hotel(deps, info, id, Some(new_name), Some(new_location)),
        ExecuteMsg::DeleteHotel { id } => try_delete_hotel(deps, info, id),
        ExecuteMsg::AddService { name, hotel_id, description } => execute_add_service(deps, info, name, hotel_id, description),
        ExecuteMsg::UpdateService { id, new_name, hotel_id, new_description } => execute_update_service(deps, info, id, Some(new_name), hotel_id, new_description),
        ExecuteMsg::AddRating { hotel_id, service_id, score, comment } => execute_add_rating(deps, info, hotel_id, service_id, score, comment),
        ExecuteMsg::UpdateRating { hotel_id, service_id, new_score, new_comment } => execute_update_rating(deps, info, hotel_id, service_id, new_score, new_comment),
    }
}

// Create a new hotel
fn try_create_hotel(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    owner_name: String,
    location: String,
) -> StdResult<Response> {
    let mut hotel_count = HOTEL_COUNT.load(deps.storage)?;
    hotel_count += 1;

    let hotel = Hotel {
        id: hotel_count,
        owner: info.sender.clone(),
        owner_name,
        name,
        location,
        services: vec![],
    };

    HOTELS.save(deps.storage, hotel_count, &hotel)?;
    HOTEL_COUNT.save(deps.storage, &hotel_count)?;

    Ok(Response::new()
        .add_attribute("method", "try_create_hotel")
        .add_attribute("id", hotel_count.to_string())
    )
}

// Update an existing hotel
fn try_update_hotel(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    new_name: Option<String>,
    new_location: Option<String>,
) -> StdResult<Response> {

    let mut hotel = HOTELS.load(deps.storage, id)?;

    if info.sender != hotel.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    if let Some(name) = new_name {
        hotel.name = name;
    }
    if let Some(location) = new_location {
        hotel.location = location;
    }

    HOTELS.save(deps.storage, id, &hotel)?;

    Ok(Response::new()
        .add_attribute("method", "try_update_hotel")
        .add_attribute("id", id.to_string()))
}

// Delete a hotel
fn try_delete_hotel(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response> {
    
    let hotel = HOTELS.load(deps.storage, id)?;
    // Only allow the owner to delete the hotel
    if hotel.owner != info.sender {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }

    HOTELS.remove(deps.storage, id);

    Ok(Response::new()
        .add_attribute("method", "try_delete_hotel")
        .add_attribute("id", id.to_string()))
}

// Add hotel service
fn execute_add_service(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    hotel_id: u64,
    description: Option<String>,
) -> StdResult<Response> {
    let mut service_count = SERVICE_COUNT.load(deps.storage)?;
    service_count += 1;

    
    let mut hotel = HOTELS.load(deps.storage, hotel_id)?;

    if hotel.owner != info.sender {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    let service = Service {
        id: service_count,
        name: name,
        hotel_id,
        ratings: vec![],
        description,
    };

    SERVICES.save(deps.storage, service_count, &service)?;
    hotel.services.push(service);
    HOTELS.save(deps.storage, hotel_id, &hotel)?;
    SERVICE_COUNT.save(deps.storage, &service_count)?;

    Ok(Response::new()
        .add_attribute("method", "execute_add_service")
        .add_attribute("hotel_id", hotel_id.to_string())
        .add_attribute("id", service_count.to_string())
    )
}

// Update hotel service
fn execute_update_service(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    new_name: Option<String>,
    hotel_id: u64,
    new_description: Option<String>,
) -> StdResult<Response> {
    
    let mut hotel = HOTELS.load(deps.storage, hotel_id)?;
    let mut service = SERVICES.load(deps.storage, id)?;

    if hotel.owner != info.sender {
        return Err(StdError::generic_err("Unauthorized"));
    }

    if let Some(name) = new_name {
        service.name = name;
    }
    
    if let Some(description) = new_description {
        service.description = Some(description);
    }

    SERVICES.save(deps.storage, id, &service)?;

    if let Some(pos) = hotel.services.iter().position(|s| s.id == id) {
        hotel.services[pos] = service.clone();
    } else {
        hotel.services.push(service.clone());
    }

    HOTELS.save(deps.storage, hotel_id, &hotel)?;

    Ok(Response::new()
        .add_attribute("method", "execute_add_service")
        // .add_attribute("hotel_id", hotel_id.to_string())
        .add_attribute("id", id.to_string())
    )
}

// Add rating to hotel
fn execute_add_rating(
    deps: DepsMut,
    info: MessageInfo,
    hotel_id: u64,
    service_id: u64,
    score: u8,
    comment: Option<String>,
) -> StdResult<Response> {
    
    if score < 1 || score > 5 {
        return Err(StdError::generic_err("Rating score must be between 1 and 5"));
    }

    // Ensure the hotel exists
    let mut hotel = HOTELS.load(deps.storage, hotel_id)?;

    // Ensure the service belongs to the hotel
    let mut service = SERVICES.load(deps.storage, service_id)?;

    if service.hotel_id != hotel_id {
        return Err(StdError::generic_err("Service does not belong to the specified hotel"));
    }

    let rating_id = (service_id, info.sender.clone());

    // Add the new rating
    let rating = Rating {
        user: info.sender.clone(),
        id: rating_id.clone(),
        hotel_id,
        service_id,
        score,
        comment,
    };

    RATINGS.save(deps.storage, rating_id.clone(), &rating)?;

    // Add the rating to the service's list of ratings
    service.ratings.push(rating);
    SERVICES.save(deps.storage, service_id, &service)?;

    if let Some(pos) = hotel.services.iter().position(|s| s.id == service_id) {
        hotel.services[pos] = service.clone();
    }

    HOTELS.save(deps.storage, hotel_id, &hotel)?;

    Ok(Response::new()
        .add_attribute("method", "add_rating")
        .add_attribute("hotel_id", hotel_id.to_string())
        .add_attribute("service_id", service_id.to_string()))
}

fn execute_update_rating(
    deps: DepsMut,
    info: MessageInfo,
    hotel_id: u64,
    service_id: u64,
    new_score: u8,
    new_comment: Option<String>,
) -> StdResult<Response> {
    if new_score < 1 || new_score > 5 {
        return Err(StdError::generic_err("Rating score must be between 1 and 5"));
    }

    // Load hotel to ensure it exists and the service belongs to it
    let mut hotel = HOTELS.load(deps.storage, hotel_id)?;
    let mut service = SERVICES.load(deps.storage, service_id)?;

    if service.hotel_id != hotel_id {
        return Err(StdError::generic_err("Service does not belong to the specified hotel"));
    }

    let rating_id = (service_id, info.sender.clone());
    let mut rating = RATINGS.load(deps.storage, rating_id.clone())?;

    // Ensure only the rating owner can update it
    if rating.user != info.sender {
        return Err(StdError::generic_err("Unauthorized"));
    }

    rating.score = new_score;
    rating.comment = new_comment;

    // Save updated rating
    RATINGS.save(deps.storage, rating_id.clone(), &rating)?;

    // Update the service's ratings
    service.ratings.retain(|r| r.id != rating_id);  // Remove old rating
    service.ratings.push(rating);                  // Add updated rating

    // Save the updated service
    SERVICES.save(deps.storage, service_id, &service)?;

    // Ensure service is correctly referenced in the hotel
    if let Some(pos) = hotel.services.iter().position(|s| s.id == service_id) {
        hotel.services[pos] = service.clone();
    } else {
        hotel.services.push(service.clone());
    }

    // Save updated hotel
    HOTELS.save(deps.storage, hotel_id, &hotel)?;

    Ok(Response::new()
        .add_attribute("method", "update_rating")
        .add_attribute("hotel_id", hotel_id.to_string())
        .add_attribute("service_id", service_id.to_string()))
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    
    match msg {
        QueryMsg::GetHotel { id } => to_json_binary(&query_hotel(deps, id)?),
        QueryMsg::GetService { id } => to_json_binary(&query_service(deps, id)?),
        QueryMsg::GetAllHotels { filter } => to_json_binary(&query_all_hotels(deps, filter)?),
    }
}

// Query a specific hotel by ID
fn query_hotel(
    deps: Deps,
    id: u64,
) -> StdResult<Hotel> {
    let hotel = HOTELS.load(deps.storage, id)?;
    Ok(hotel)
}

// Query a specific service by ID
fn query_service(
    deps: Deps,
    id: u64
) -> StdResult<Service> {
    let service = SERVICES.load(deps.storage, id)?;
    Ok(service)
}

// Query all hotels
fn query_all_hotels(deps: Deps, filter: Option<String>) -> StdResult<AllHotelsResponse> {
    let all: StdResult<Vec<_>> = HOTELS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| {
            let (_key, hotel) = item.ok()?;
            if filter.as_ref().map_or(true, |f| hotel.name.contains(f) || hotel.location.contains(f)) {
                Some(Ok(hotel))
            } else {
                None
            }
        })
        .collect();

    all.map(|hotels| AllHotelsResponse { hotels })
}