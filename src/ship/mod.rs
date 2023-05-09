mod components;
mod systems;

use bevy::prelude::*; 
use crate::ship::systems::*;
use crate::ship::components::*;
//use crate::analizer::*;


pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ship>();
        app.register_type::<ShipStatus>();
        app.register_type::<Detector>();
        app.add_startup_system(create_ships);
        app.add_system(update_ships);
        app.add_system(wrap_elements);
        app.add_system(scan_for_contacts);
    }
}