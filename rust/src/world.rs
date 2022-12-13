use std::time::SystemTime;
use gdnative::api::*;
use gdnative::prelude::*;
use uuid::Uuid;

use crate::common::{Coordinates, Location, MobileLocation};
use crate::assets::{Vertiport, VertiportStatus, Aircraft, AircraftStatus};
// use gdnative::api::xml_parser::NodeType;
// use std::str::FromStr;

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_builder)]
pub struct World {
    // meshes: Ref<MeshLibrary, Unique>,
    vertiports: Vec<Vertiport>,
    aircraft: Vec<Aircraft>,
}

#[methods]
impl World {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("World builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Spatial) -> Self {
        World {
            vertiports: vec![],
            aircraft: vec![],
            // tfrs: vec![]
        }
    }

    fn get_vertiports(&mut self) /* return Result */
    {
        self.vertiports.push(Vertiport::new(
            Uuid::new_v4(),
            VertiportStatus::ACTIVE,
            "East Harrow".into(),
            Location::Static(Coordinates { latitude: -12.9912682, longitude: 33.4617457 })
        ));

        self.vertiports.push(Vertiport::new(
            Uuid::new_v4(),
            VertiportStatus::INACTIVE,
            "East Harrow".into(),
            Location::Mobile(
                MobileLocation {
                    last_known_coordinates: Some(Coordinates { latitude: -12.9912682, longitude: 33.4617457 }),
                    last_update: Some(SystemTime::now())
                }
            )
        ));
    }

    fn get_aircraft(&mut self) /* return Result */
    {
        // TODO: Replace struct and hardcode with REST call to svc-cargo
        self.aircraft.push(Aircraft::new(
            Uuid::new_v4(),
            AircraftStatus::ACTIVE,
            "Victor 99".into()
        ));
        self.aircraft.push(Aircraft::new(
            Uuid::new_v4(),
            AircraftStatus::ACTIVE,
            "Foxtrot S10".into()
        ));
    }

    fn populate_meshlib(&mut self) {
        println!("Populating meshes...");
        todo!();
        // aircraft
        // vertiports
    }

    fn initialize(&mut self, _owner: &Spatial) {
        // Populate Meshes
        self.populate_meshlib();

        // Get Assets
        self.get_vertiports();
        self.get_aircraft();

        // Load Background Image from OpenStreetMaps
        // let sample = GodotString::from_str("res://assets/osm/malawi.osm");
        // self.load_osm(owner, sample);
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &Spatial) {
        self.initialize(owner);
        godot_print!("World Ready");
    }

    // fn osm_get_bounds(&mut self, _owner: &Spatial, _parser: &XMLParser) {
    //     // let minlat = parser.get_named_attribute_value("minlat").to_f32();
    //     // let maxlat = parser.get_named_attribute_value("maxlat").to_f32();
    //     // let minlon = parser.get_named_attribute_value("minlon").to_f32();
    //     // let maxlon = parser.get_named_attribute_value("maxlon").to_f32();

    //     // Start location is center of map
    //     // owner.set_center_x((1000. * (minlat + maxlat) / 2.) as i64);
    //     // owner.set_center_y((1000. * (minlon + maxlon) / 2.) as i64);
    //     // owner.set_center_z(0);
    // }

    // fn osm_get_node(&mut self, _owner: &Spatial, parser: &XMLParser) {
    //     let _lat = parser.get_named_attribute_value("lat").to_f32();
    //     let _lon = parser.get_named_attribute_value("lon").to_f32();
    //     let id = parser.get_named_attribute_value("id").to_string();
    //     let _id: i64 = i64::from_str(&id).unwrap();

    //     // owner.set_cell_item((lat*1000.) as i64, (lon*1000.) as i64, 0, 0, 0);
    //     // self.nodes.insert(id, Coordinates { lat, lon });
    // }

    // fn osm_get_way(&mut self, _owner: &Spatial, parser: &XMLParser) {
    //     let id = parser.get_named_attribute_value("id").to_string();
    //     let _id: i64 = i64::from_str(&id).unwrap();

    //     let _ref_str: GodotString = GodotString::from("ref");
    //     let _tag_str: GodotString = GodotString::from("tag");
    //     while parser.get_node_type() != NodeType::ELEMENT_END {
    //         if parser.read().is_err() {
    //             panic!("reached EOF before end element");
    //         }

    //         todo!();

    //         // match parser.get_node_name() {
    //         //     ref_str => {

    //         //     },

    //         //     tag_str => {

    //         //     },

    //         //     e => {
    //         //         panic!("Unexpected tag! {:?}", e);
    //         //     }
    //         // }
    //     }
    // }

    // fn osm_get_relation(&mut self, _owner: &Spatial, parser: &XMLParser) {
    //     let id = parser.get_named_attribute_value("id").to_string();
    //     let _id: i64 = i64::from_str(&id).unwrap();

    //     let _member_str: GodotString = GodotString::from("member");
    //     let _tag_str: GodotString = GodotString::from("tag");

    //     while parser.get_node_type() != NodeType::ELEMENT_END {
    //         if parser.read().is_err() {
    //             panic!("reached EOF before end element");
    //         }

    //         todo!();

    //         // match parser.get_node_name() {
    //         //     member_str => {

    //         //     },

    //         //     tag_str => {

    //         //     },

    //         //     e => {
    //         //         panic!("Unexpected tag! {:?}", e);
    //         //     }
    //         // }
    //     }

    //     // create entities
    // }

    // pub fn load_osm(&mut self, owner: &Spatial, osm_filepath: GodotString) -> bool {
    //     let parser = XMLParser::new();
    //     if parser.open(osm_filepath).is_err() {
    //         println!("Could not open OSM file.");
    //         // debug!("Could not open OSM file: {}", osm_filepath)
    //         return false;
    //     };

    //     let bounds_str: GodotString = GodotString::from("bounds");
    //     let way_str: GodotString = GodotString::from("way");
    //     let node_str: GodotString = GodotString::from("node");
    //     let relation_str: GodotString = GodotString::from("relation");

    //     while parser.read().is_ok() {
    //         if parser.get_node_type() != NodeType::ELEMENT {
    //             continue;
    //         }

    //         match parser.get_node_name() {
    //             y if y == bounds_str => self.osm_get_bounds(owner, &parser),
    //             y if y == node_str => self.osm_get_node(owner, &parser),
    //             y if y == way_str => self.osm_get_way(owner, &parser),
    //             y if y == relation_str => self.osm_get_relation(owner, &parser),
    //             e => {
    //                 println!("other: {:?}", e);
    //             }
    //         }
    //     }

    //     true
    // }
}
