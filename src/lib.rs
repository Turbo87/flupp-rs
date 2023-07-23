use std::convert::Infallible;
use std::str::FromStr;
use time::{Date, Duration, Time};

const GEN_SETTINGS_PREFIX: &str = "[GenSettings]";
const LIC_SETTINGS_PREFIX: &str = "[LicSettings]";
const TABLE_COLS_PREFIX: &str = "[TableCols]";

#[derive(Debug, Clone)]
pub struct File {
    pub general_settings: Option<GeneralSettings>,
    pub flight_logs: Vec<FlightLog>,
}

impl File {
    pub fn from_reader<R: std::io::Read>(mut reader: R) -> Result<Self, Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Self::from_slice(&buffer)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let (cow, _encoding, _had_errors) = encoding_rs::WINDOWS_1252.decode(bytes);
        Self::from_str(&cow)
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (version, rest) = content.split_once('\n').ok_or(Error::InvalidFile)?;
        if version.trim() != "4" {
            return Err(Error::UnsupportedFileVersion(version.to_string()));
        }

        let (header, rest) = rest.split_once("\n:").ok_or(Error::InvalidFile)?;

        let general_settings = header
            .lines()
            .rfind(|line| line.starts_with(GEN_SETTINGS_PREFIX))
            .map(|line| GeneralSettings::from_str(&line[GEN_SETTINGS_PREFIX.len()..]).unwrap());

        let flight_logs = rest
            .split("\n:")
            .map(FlightLog::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            general_settings,
            flight_logs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeneralSettings {
    pub name: String,
    pub road: String,
    pub location: String,
    pub pilot_name: String,
}

impl FromStr for GeneralSettings {
    type Err = Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut split_iter = content.split(';');
        Ok(Self {
            name: split_iter.next().unwrap().to_string(),
            road: split_iter.next().unwrap_or_default().to_string(),
            location: split_iter.next().unwrap_or_default().to_string(),
            pilot_name: split_iter.next().unwrap_or_default().to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FlightLog {
    pub title: String,
    pub license_settings: LicenseSettings,
    pub aircraft_ids: Vec<String>,
    pub aircraft_types: Vec<String>,
    pub copilots: Vec<String>,
    pub locations: Vec<String>,
    pub column_widths: Vec<String>,
    pub time_categories: Vec<String>,
    pub categories: Vec<String>,
    pub contest_categories: Vec<String>,
    pub license_categories: Vec<String>,
    pub license_time_categories: Vec<String>,
    pub license_dates: Vec<String>,
    pub acc_licenses: Vec<String>,
    pub opt_conditions: Vec<String>,
    pub flights: Vec<Flight>,
}

impl FromStr for FlightLog {
    type Err = Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (header, csv) = content
            .split_once(TABLE_COLS_PREFIX)
            .ok_or(Error::InvalidFile)?;

        let mut header_lines_iter = header.lines();

        let title = header_lines_iter.next().ok_or(Error::InvalidFile)?;
        let title = title
            .split_once(';')
            .map(|(title, _)| title)
            .unwrap_or(title)
            .to_string();

        let license_settings = header_lines_iter.next().ok_or(Error::MissingLicSettings)?;
        if !license_settings.starts_with(LIC_SETTINGS_PREFIX) {
            return Err(Error::MissingLicSettings);
        }

        let license_settings = &license_settings[LIC_SETTINGS_PREFIX.len()..];
        let license_settings = LicenseSettings::from_str(license_settings)?;

        let header_lines = header_lines_iter.collect::<Vec<_>>();

        let aircraft_ids = find_list(&header_lines, "[AId]");
        let aircraft_types = find_list(&header_lines, "[AType]");
        let copilots = find_list(&header_lines, "[CoPilot]");
        let locations = find_list(&header_lines, "[Loc]");
        let column_widths = find_list(&header_lines, "[ColWidth]");
        let time_categories = find_list(&header_lines, "[CatTime]");
        let categories = find_list(&header_lines, "[Category]");
        let contest_categories = find_list(&header_lines, "[Contest]");
        let license_categories = find_list(&header_lines, "[LicenseCat]");
        let license_time_categories = find_list(&header_lines, "[LicenseTimeCat]");
        let license_dates = find_list(&header_lines, "[LicenseDates]");
        let acc_licenses = find_list(&header_lines, "[AccLicenses]");
        let opt_conditions = find_list(&header_lines, "[OptConditions]");

        Ok(Self {
            title,
            license_settings,
            aircraft_ids: aircraft_ids.unwrap_or_default(),
            aircraft_types: aircraft_types.unwrap_or_default(),
            copilots: copilots.unwrap_or_default(),
            locations: locations.unwrap_or_default(),
            column_widths: column_widths.unwrap_or_default(),
            time_categories: time_categories.unwrap_or_default(),
            categories: categories.unwrap_or_default(),
            contest_categories: contest_categories.unwrap_or_default(),
            license_categories: license_categories.unwrap_or_default(),
            license_time_categories: license_time_categories.unwrap_or_default(),
            license_dates: license_dates.unwrap_or_default(),
            acc_licenses: acc_licenses.unwrap_or_default(),
            opt_conditions: opt_conditions.unwrap_or_default(),
            flights: parse_flights(csv)?,
        })
    }
}

fn find_list(lines: &[&str], prefix: &str) -> Option<Vec<String>> {
    let line = lines.iter().rfind(|line| line.starts_with(prefix))?;
    let content = &line[prefix.len()..];
    let list = content
        .split(';')
        .filter(|str| !str.is_empty())
        .map(ToString::to_string)
        .collect();

    Some(list)
}

#[derive(Debug, Clone)]
pub struct LicenseSettings {
    pub bf_starts: u32,
    pub bf_time: Duration,
    pub license_since: Option<Date>,
    pub id_prefix: String,
    pub distance_unit: String,
}

impl FromStr for LicenseSettings {
    type Err = Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut split_iter = content.split(';');

        let bf_starts = split_iter.next().unwrap();
        let bf_starts = bf_starts
            .parse()
            .map_err(|_| Error::InvalidLicSettings(content.to_string()))?;

        let bf_time = split_iter
            .next()
            .map(parse_duration)
            .transpose()?
            .unwrap_or_default();

        let license_since = split_iter.nth(1).map(parse_date).transpose()?;

        let id_prefix = split_iter.next().unwrap_or_default().to_string();
        let distance_unit = split_iter.nth(3).unwrap_or_default().to_string();

        Ok(Self {
            bf_starts,
            bf_time,
            license_since,
            id_prefix,
            distance_unit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Flight {
    pub number: Option<u32>,
    pub date: Date,
    pub aircraft_type: String,
    pub aircraft_id: String,
    pub pilot_name: String,
    pub copilot_name: String,
    pub num_passengers: u8,
    pub start_type: String,
    pub start_time: Time,
    pub landing_time: Time,
    pub flight_time: Duration,
    pub block_on_time: Option<Time>,
    pub block_off_time: Option<Time>,
    pub block_time: Option<Duration>,
    pub num_landings: u16,
    pub start_location: String,
    pub landing_location: String,
    pub remarks: String,
    pub distance: Option<f32>,
    pub categories: Vec<String>,
}

fn parse_flights(content: &str) -> Result<Vec<Flight>, Error> {
    let mut lines_iter = content.lines();

    let header = lines_iter.next().ok_or(Error::InvalidFile)?;
    let header = header.split(';').collect::<Vec<_>>();

    let flights = lines_iter
        .map(|line| parse_flight(line, &header))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(flights)
}

fn parse_flight(line: &str, header: &[&str]) -> Result<Flight, Error> {
    let fields = line.split(';').collect::<Vec<_>>();
    if fields.len() != header.len() {
        return Err(Error::InvalidFile);
    }

    let mut number = Default::default();
    let mut date = None;
    let mut aircraft_type = Default::default();
    let mut aircraft_id = Default::default();
    let mut pilot_name = Default::default();
    let mut copilot_name = Default::default();
    let mut num_passengers = Default::default();
    let mut start_type = Default::default();
    let mut start_time = None;
    let mut landing_time = None;
    let mut flight_time = Default::default();
    let mut block_on_time = Default::default();
    let mut block_off_time = Default::default();
    let mut block_time = Default::default();
    let mut num_landings = 1;
    let mut start_location = Default::default();
    let mut landing_location = Default::default();
    let mut remarks = Default::default();
    let mut distance = Default::default();
    let mut categories = Default::default();

    for (&header, value) in header.iter().zip(fields) {
        match header {
            "Num" => number = Some(value.parse().map_err(|_| Error::InvalidFile)?),
            "Dat" => date = Some(parse_date(value)?),
            "ATy" | "Mus" => aircraft_type = value.to_string(),
            "AId" | "Ken" => aircraft_id = value.to_string(),
            "Pi1" | "Pil" => pilot_name = value.to_string(),
            "Pi2" | "Beg" => copilot_name = value.to_string(),
            "Pas" => num_passengers = value.parse().map_err(|_| Error::InvalidFile)?,
            "ToS" | "Art" => start_type = value.to_string(),
            "StT" | "StZ" => start_time = Some(parse_time(value)?),
            "LaT" | "LaZ" => landing_time = Some(parse_time(value)?),
            "FlT" | "FlZ" => flight_time = parse_duration(value)?,
            "StB" => block_on_time = Some(parse_time(value)?),
            "LaB" => block_off_time = Some(parse_time(value)?),
            "BlT" | "BlZ" => block_time = Some(parse_duration(value)?),
            "NoL" | "AFl" => num_landings = value.parse().map_err(|_| Error::InvalidFile)?,
            "StL" | "StO" => start_location = value.to_string(),
            "LaL" | "LaO" => landing_location = value.to_string(),
            "Rem" | "Bem" => remarks = value.to_string(),
            "Dst" | "Str" if !value.is_empty() => {
                distance = Some(value.parse().map_err(|_| Error::InvalidFile)?)
            }
            "Cat" | "Kat" => {
                categories = value
                    .split('/')
                    .filter(|str| !str.is_empty())
                    .map(ToString::to_string)
                    .collect()
            }
            _ => {}
        }
    }

    let flight = Flight {
        number,
        date: date.ok_or(Error::InvalidFile)?,
        aircraft_type,
        aircraft_id,
        pilot_name,
        copilot_name,
        num_passengers,
        start_type,
        start_time: start_time.ok_or(Error::InvalidFile)?,
        landing_time: landing_time.ok_or(Error::InvalidFile)?,
        flight_time,
        block_on_time,
        block_off_time,
        block_time,
        num_landings,
        start_location,
        landing_location,
        remarks,
        distance,
        categories,
    };

    Ok(flight)
}

fn parse_duration(str: &str) -> Result<Duration, Error> {
    use Error::InvalidDuration;

    let (hours, minutes) = str.split_once(':').unwrap_or((str, "0"));

    let hours = hours
        .parse()
        .map_err(|_| InvalidDuration(str.to_string()))?;
    let minutes = minutes
        .parse()
        .map_err(|_| InvalidDuration(str.to_string()))?;

    Ok(Duration::hours(hours) + Duration::minutes(minutes))
}

fn parse_date(str: &str) -> Result<Date, Error> {
    use Error::InvalidDate;

    let (dd, rest) = str
        .split_once('.')
        .ok_or_else(|| InvalidDate(str.to_string()))?;
    let (mm, yy) = rest
        .split_once('.')
        .ok_or_else(|| InvalidDate(str.to_string()))?;

    let yy = yy.parse().map_err(|_| InvalidDate(str.to_string()))?;
    let mm: u8 = mm.parse().map_err(|_| InvalidDate(str.to_string()))?;
    let mm = mm.try_into().map_err(|_| InvalidDate(str.to_string()))?;
    let dd = dd.parse().map_err(|_| InvalidDate(str.to_string()))?;

    Date::from_calendar_date(yy, mm, dd).map_err(|_| InvalidDate(str.to_string()))
}

fn parse_time(str: &str) -> Result<Time, Error> {
    use Error::InvalidTime;

    let (hh, mm) = str
        .split_once(':')
        .ok_or_else(|| InvalidTime(str.to_string()))?;

    let hh = hh.parse().map_err(|_| InvalidTime(str.to_string()))?;
    let mm = mm.parse().map_err(|_| InvalidTime(str.to_string()))?;

    Time::from_hms(hh, mm, 0).map_err(|_| InvalidTime(str.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported FluPP file version: {0}")]
    UnsupportedFileVersion(String),
    #[error("Invalid FluPP file")]
    InvalidFile,
    #[error("Missing [LicSettings] line")]
    MissingLicSettings,
    #[error("Invalid [LicSettings] line: {0}")]
    InvalidLicSettings(String),
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Invalid time: {0}")]
    InvalidTime(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = include_bytes!("../data/flugbuch.flu");
        insta::assert_debug_snapshot!(File::from_slice(bytes));
    }
}
