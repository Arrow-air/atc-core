use std::time::SystemTime;

pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct MobileLocation {
    pub last_known_coordinates: Option<Coordinates>,
    pub last_update: Option<SystemTime>
}

pub enum Location {
    Static(Coordinates),
    Mobile(MobileLocation)
}
