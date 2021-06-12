use bevy::prelude::{
    EventReader, 
    ResMut
};
use bevy_mod_picking::{
    PickingEvent, 
    SelectionEvent
};

use super::OrbitTarget;

pub(crate) fn select_orbits(
    mut events: EventReader<PickingEvent>, 
    mut select_target: ResMut<OrbitTarget>,
    // q: Query<(Entity, &Transform), With<OrbitalBody>>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(selection) => {
                match selection {

                    // Selection
                    SelectionEvent::JustSelected(selected_entity) => {
                        match select_target.body {
                            Some(existing) if existing != *selected_entity => select_target.body = Some(*selected_entity),
                            Some(_) => (),
                            None => select_target.body = Some(*selected_entity),
                        }
                    },

                    // Deselection
                    SelectionEvent::JustDeselected(deselected_entity) => {
                        match select_target.body {
                            Some(existing) if existing == *deselected_entity => select_target.body = None,
                            Some(_) => (),
                            None => (),
                        }
                    }
                }
            },
            PickingEvent::Hover(_) => (),
        }
    }
}
