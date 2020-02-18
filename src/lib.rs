use phf::phf_map;
use serde::Deserialize;

pub struct Line {
    pub name: &'static str,
    pub color: &'static str,
}

pub static LINES: phf::Map<&'static str, Line> = phf_map! {
    "BL" => Line { name: "Blue", color: "#0079C4" },
    "GR" => Line { name: "Green", color: "#10AE4E" },
    "OR" => Line { name: "Orange", color: "#FF9027" },
    "RD" => Line { name: "Red", color: "#FA2C2E" },
    "SV" => Line { name: "Silver", color: "#9AA7A3" },
    "YL" => Line { name: "Yellow", color: "#FFCE36" }
};

pub static STATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "G03" => "Addison Road-Seat Pleasant",
    "F06" => "Anacostia",
    "F02" => "Archives-Navy Memorial-Penn Quarter",
    "C06" => "Arlington Cemetery",
    "K04" => "Ballston-MU",
    "G01" => "Benning Road",
    "A09" => "Bethesda",
    "C12" => "Braddock Road",
    "F11" => "Branch Ave",
    "B05" => "Brookland-CUA",
    "G02" => "Capitol Heights",
    "D05" => "Capitol South",
    "D11" => "Cheverly",
    "K02" => "Clarendon",
    "A05" => "Cleveland Park",
    "E09" => "College Park-U of Md",
    "E04" => "Columbia Heights",
    "F07" => "Congress Heights",
    "K01" => "Court House",
    "C09" => "Crystal City",
    "D10" => "Deanwood",
    "K07" => "Dunn Loring-Merrifield",
    "A03" => "Dupont Circle",
    "K05" => "East Falls Church",
    "D06" => "Eastern Market",
    "C14" => "Eisenhower Avenue",
    "A02" => "Farragut North",
    "C03" => "Farragut West",
    "D04" => "Federal Center SW",
    "D01" => "Federal Triangle",
    "C04" => "Foggy Bottom-GWU",
    "B09" => "Forest Glen",
    "B06,E06" => "Fort Totten",
    "J03" => "Franconia-Springfield",
    "A08" => "Friendship Heights",
    "B01,F01" => "Gallery Pl-Chinatown",
    "E05" => "Georgia Ave-Petworth",
    "B11" => "Glenmont",
    "E10" => "Greenbelt",
    "N03" => "Greensboro",
    "A11" => "Grosvenor-Strathmore",
    "C15" => "Huntington",
    "B02" => "Judiciary Square",
    "C13" => "King St-Old Town",
    "D03,F03" => "L'Enfant Plaza",
    "D12" => "Landover",
    "G05" => "Largo Town Center",
    "N01" => "McLean",
    "C02" => "McPherson Square",
    "A10" => "Medical Center",
    "A01,C01" => "Metro Center",
    "D09" => "Minnesota Ave",
    "G04" => "Morgan Boulevard",
    "E01" => "Mt Vernon Sq 7th St-Convention Center",
    "F05" => "Navy Yard-Ballpark",
    "F09" => "Naylor Road",
    "D13" => "New Carrollton",
    "B35" => "NoMa-Gallaudet U",
    "C07" => "Pentagon",
    "C08" => "Pentagon City",
    "D07" => "Potomac Ave",
    "E08" => "Prince George's Plaza",
    "B04" => "Rhode Island Ave-Brentwood",
    "A14" => "Rockville",
    "C10" => "Ronald Reagan Washington National Airport",
    "C05" => "Rosslyn",
    "A15" => "Shady Grove",
    "E02" => "Shaw-Howard U",
    "B08" => "Silver Spring",
    "T81" => "Silver Spring Transit Center",
    "D02" => "Smithsonian",
    "F08" => "Southern Avenue",
    "N04" => "Spring Hill",
    "D08" => "Stadium-Armory",
    "F10" => "Suitland",
    "B07" => "Takoma",
    "A07" => "Tenleytown-AU",
    "A13" => "Twinbrook",
    "N02" => "Tysons Corner",
    "E03" => "U Street/African-Amer Civil War Memorial/Cardozo",
    "B03" => "Union Station",
    "J02" => "Van Dorn Street",
    "A06" => "Van Ness-UDC",
    "K08" => "Vienna/Fairfax-GMU",
    "K03" => "Virginia Square-GMU",
    "F04" => "Waterfront",
    "K06" => "West Falls Church-VT/UVA",
    "E07" => "West Hyattsville",
    "B10" => "Wheaton",
    "A12" => "White Flint",
    "N06" => "Wiehle-Reston East",
    "A04" => "Woodley Park-Zoo/Adams Morgan"
};

#[derive(Deserialize)]
pub struct Train {
    #[serde(rename = "Group")]
    pub group: i8,
    #[serde(rename = "Min")]
    pub min: serde_json::Value,
    #[serde(rename = "Car")]
    pub cars: serde_json::Value,
    #[serde(rename = "Line")]
    pub line: String,
    #[serde(rename = "Destination")]
    pub destination: String,
}

#[derive(Deserialize)]
pub struct Trains {
    #[serde(rename = "TRAINS")]
    pub trains: Vec<Train>,
}

pub fn get_next_trains(station: &str) -> Result<Vec<Train>, reqwest::Error> {
    let url = format!(
        "https://www.wmata.com/components/stations.cfc?method=getNextTrains&StationCode={}&returnFormat=JSON",
        station
    );

    let response: Trains = reqwest::get(&url)?.json()?;
    return Ok(response.trains);
}
