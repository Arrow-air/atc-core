use crate::common::{MobileLocation, Location};
use uuid::Uuid;
use gdnative::api::*;
use gdnative::prelude::*;

// TODO: Replace struct and hardcode with REST call to svc-cargo
pub enum AircraftStatus {
    ACTIVE
}

// const AIRCRAFT_FLIGHT: &str = "res://assets/aircraft/aircraft-active-64x64.png";
// const AIRCRAFT_GROUND: &str = "res://assets/aircraft/aircraft-ground-64x64.png";
// const AIRCRAFT_CARGO: &str = "res://assets/aircraft/aircraft-cargo-64x64.png";
// const AIRCRAFT_INACTIVE: &str = "res://assets/aircraft/aircraft-inactive-64x64.png";
#[allow(dead_code)]
pub struct Aircraft {
    uuid: Uuid,
    polygon: Ref<Polygon2D, Unique>,
    label: String,
    location: Location,
    status: AircraftStatus,
}

impl Aircraft {
    pub fn new(uuid: Uuid, status: AircraftStatus, label: String) -> Self {
        let poly = Polygon2D::new();
        poly.set_polygon(
            [
                Vector2::new(-32.,-32.),
                Vector2::new(32., -32.),
                Vector2::new(32., 32.),
                Vector2::new(-32., 32.),

            ].into_iter().collect::<PoolArray<Vector2>>()
        );

        poly.set_color(Color { a: 1., b: 1., g: 1., r: 1. });

        Aircraft {
            uuid: uuid,
            polygon: poly,
            label: label,
            location: Location::Mobile(
                MobileLocation {
                    last_known_coordinates: None,
                    last_update: None
                }
            ),
            status: status
        }
    }

    // TODO: Red/Green Color Blindness/Accessibility Settings
    // pub fn get_mesh(&mut self) -> Ref<Polygon2D, Unique> {
    //     // match status {
    //     //     ACTIVE => poly.set_polygon,
    //     //     ACTIVE_CARGO => ,
    //     //     GROUND => ,
    //     //     INACTIVE =>
    //     // }

    //     // poly
    //     // self.polygon
    // }
}


pub enum VertiportStatus {
    ACTIVE,
    INACTIVE
}

// const VERTIPAD_ACTIVE: &str = "res://assets/vertiport/vertipad-active-64x64.png";
// const VERTIPAD_INACTIVE: &str = "res://assets/vertiport/vertipad-inactive-64x64.png";
#[allow(dead_code)]
pub struct Vertiport {
    uuid: Uuid,
    label: String,
    status: VertiportStatus,
    polygon: Ref<Polygon2D, Unique>,
    location: Location,
}

impl Vertiport {
    pub fn new(uuid: Uuid, status: VertiportStatus, label: String, location: Location) -> Self {
        let poly = Polygon2D::new();
        poly.set_polygon(
            [
                Vector2::new(-32.,-32.),
                Vector2::new(32., -32.),
                Vector2::new(32., 32.),
                Vector2::new(-32., 32.),

            ].into_iter().collect::<PoolArray<Vector2>>()
        );

        poly.set_color(Color { a: 1., b: 1., g: 1., r: 1.});
        Vertiport {
            uuid: uuid,
            polygon: poly,
            label: label,
            location: location,
            status: status
        }
    }
}