mod select_orbits;

use self::select_orbits::select_orbits;

use bevy::prelude::*;
use bevy_mod_picking::{
    HighlightablePickingPlugin, 
    InteractablePickingPlugin, 
    PickingPlugin
};

/// Plugin responsible for making orbital bodies pickable 
/// so that they can be interacted with in other plugins and systems
pub struct OrbitPickerPlugin;

impl Plugin for OrbitPickerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(OrbitTarget { selection: None, hover: None })
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(HighlightablePickingPlugin)

            .add_system(select_orbits.system())
        ;
    }
}

pub struct OrbitTarget {
    pub selection: Option<Entity>,
    pub hover: Option<Entity>,
}