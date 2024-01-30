use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell}; 


//weather condition struct for deserializing response
#[derive(CandidType, Deserialize, Clone)]
#[derive(Default, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub id:f64,
    pub main: String,
    pub description: String,
}

//wind info struct for deserializing response
#[derive(CandidType, Deserialize, Clone)]
#[derive(Default, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
}

// main data struct for deserializing response
#[derive(CandidType, Deserialize, Clone)]
#[derive(Default, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: f64,
}

//base struct
#[derive(CandidType, Deserialize, Clone)]
#[derive(Default, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub main: Main,
    pub weather: Vec<Condition>,
    pub wind:Wind

}

// Implementing Storable for Weather
impl Storable for Weather {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

//memory definition
type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

// Implementing BoundedStorable for Weather
impl BoundedStorable for Weather {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

// Creating memory manager with a new MemoryId
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static WEATHER_MAP: RefCell<StableBTreeMap<u64, Weather,Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // Use a different MemoryId if needed
        )
    );
}


//getting weather data by city name
#[ic_cdk::update]
async fn get_weather_by_city_name(city_name: String) -> Result<Weather,String> {
    let api_endpoint = "https://api.openweathermap.org/data/2.5/weather?q=";
    let new_city_name = city_name.replace(" ", "%20");

    //key value to access
    let api_key = "a192e3072dae06165a3e7ec914bfa215";

    // url to be sent
    let url = format!("{}{}&units=metric&APPID={}", api_endpoint, new_city_name, api_key);

    let request_headers = vec![];

    // setting the request params,headers and body
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    //if Ok => deserializing response
    //else => returning a string message
    match http_request(request).await {
        Ok((response,)) => {
            if response.status == 200 {
                let weather_data: Weather =
                    serde_json::from_slice(&response.body).expect("Failed to parse JSON response.");
                
                return Ok(weather_data);
            } else {
                Err(format!("HTTP request failed with status code: {}", response.status))
            }
        }
        Err((code, message)) => {
            Err(format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            ))
        }
    }
}
