use time::{Date, Duration, Time};

#[derive(Debug, Clone)]
pub struct File {
    general_settings: GeneralSettings,
    flight_logs: Vec<FlightLog>,
}

#[derive(Debug, Clone)]
pub struct GeneralSettings {
    name: String,
    road: String,
    location: String,
    pilot_name: String,
}

#[derive(Debug, Clone)]
pub struct FlightLog {
    license_settings: LicenseSettings,
    aircraft_ids: Vec<String>,
    aircraft_types: Vec<String>,
    copilots: Vec<String>,
    locations: Vec<String>,
    column_widths: Vec<u16>,
    time_categories: Vec<String>,
    categories: Vec<String>,
    contest_categories: Vec<String>,
    license_categories: Vec<String>,
    license_time_categories: Vec<String>,
    license_dates: Vec<String>,
    acc_licenses: Vec<String>,
    opt_conditions: Vec<String>,
    flights: Vec<Flight>,
}

#[derive(Debug, Clone)]
pub struct LicenseSettings {
    bf_starts: u32,
    bf_time: Duration,
    license_since: Date,
    id_prefix: String,
    distance_unit: String,
}

#[derive(Debug, Clone)]
pub struct Flight {
    number: u32,
    date: Date,
    aircraft_type: String,
    aircraft_id: String,
    pilot_name: String,
    copilot_name: String,
    num_passengers: u8,
    start_type: String,
    start_time: Time,
    landing_time: Time,
    flight_time: Duration,
    block_on_time: Time,
    block_off_time: Time,
    block_time: Duration,
    num_landings: u16,
    start_location: String,
    landing_location: String,
    remarks: String,
    distance: f32,
    categories: Vec<String>,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
