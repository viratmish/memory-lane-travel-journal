#[macro_use]
extern crate serde;
use candid::{Decode, Encode};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TravelExperience {
    id: u64,
    destination: String,
    date: u64,
    notes: String,
    historical_events: Vec<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TravelExperiencePayload {
    destination: String,
    date: u64,
    notes: String,
    historical_events: Vec<String>,
}

impl Storable for TravelExperience {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TravelExperience {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Existing thread-local variables
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static TRAVEL_EXPERIENCE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for travel experiences")
    );

    static TRAVEL_EXPERIENCE_STORAGE: RefCell<StableBTreeMap<u64, TravelExperience, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for TravelExperience
fn do_insert_travel_experience(item: &TravelExperience) {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        service.borrow_mut().insert(item.id, item.clone());
    });
}

#[ic_cdk::query]
fn get_travel_experience(id: u64) -> Result<TravelExperience, Error> {
    match _get_travel_experience(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("travel experience with id={} not found", id),
        }),
    }
}

fn _get_travel_experience(id: &u64) -> Option<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_travel_experience(item: TravelExperiencePayload) -> Option<TravelExperience> {
    let id = TRAVEL_EXPERIENCE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for travel experiences");
    let travel_experience = TravelExperience {
        id,
        destination: item.destination,
        date: item.date,
        notes: item.notes,
        historical_events: item.historical_events,
    };
    do_insert_travel_experience(&travel_experience);
    Some(travel_experience)
}

#[ic_cdk::update]
fn update_travel_experience(id: u64, item: TravelExperiencePayload) -> Result<TravelExperience, Error> {
    match TRAVEL_EXPERIENCE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut travel_experience) => {
            travel_experience.destination = item.destination;
            travel_experience.date = item.date;
            travel_experience.notes = item.notes;
            travel_experience.historical_events = item.historical_events;
            do_insert_travel_experience(&travel_experience);
            Ok(travel_experience)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update travel experience with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_travel_experience(id: u64) -> Result<TravelExperience, Error> {
    match TRAVEL_EXPERIENCE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(travel_experience) => Ok(travel_experience),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete travel experience with id={}. item not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

#[ic_cdk::query]
fn get_travel_experiences_before_date(date: u64) -> Vec<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, experience)| experience.date <= date)
            .map(|(_, experience)| experience.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_travel_experience_date(id: u64, new_date: u64) -> Result<TravelExperience, Error> {
    match TRAVEL_EXPERIENCE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut travel_experience) => {
            travel_experience.date = new_date;
            do_insert_travel_experience(&travel_experience);
            Ok(travel_experience)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update date for travel experience with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_travel_experiences() -> Vec<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_total_travel_experiences() -> u64 {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| service.borrow().len())
}

#[ic_cdk::query]
fn get_travel_experiences_count_before_date(date: u64) -> usize {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, experience)| experience.date <= date)
            .count()
    })
}

#[ic_cdk::query]
fn search_travel_experiences_by_destination(destination: String) -> Vec<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, experience)| experience.destination == destination)
            .map(|(_, experience)| experience.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_sorted_travel_experiences_by_date() -> Vec<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        let mut experiences: Vec<_> = service
            .borrow()
            .iter()
            .map(|(_, experience)| experience.clone())
            .collect();

        experiences.sort_by_key(|experience| experience.date);
        experiences
    })
}

#[ic_cdk::query]
fn get_latest_travel_experiences(count: usize) -> Vec<TravelExperience> {
    TRAVEL_EXPERIENCE_STORAGE.with(|service| {
        let mut experiences: Vec<_> = service
            .borrow()
            .iter()
            .map(|(_, experience)| experience.clone())
            .collect();

        experiences.sort_by_key(|experience| experience.date);
        experiences.reverse(); // Sort in descending order (latest first)
        experiences.into_iter().take(count).collect()
    })
}


// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
