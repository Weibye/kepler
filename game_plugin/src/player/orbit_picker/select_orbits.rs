use bevy::prelude::{
    EventReader, 
    ResMut
};
use bevy_mod_picking::{HoverEvent, PickingEvent, SelectionEvent};

use super::OrbitTarget;

pub(crate) fn select_orbits(
    mut events: EventReader<PickingEvent>, 
    mut select_target: ResMut<OrbitTarget>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(selection) => {
                match selection {

                    // Selection
                    SelectionEvent::JustSelected(selected_entity) => {
                        match select_target.selection {
                            Some(existing) if existing != *selected_entity => select_target.selection = Some(*selected_entity),
                            Some(_) => (),
                            None => select_target.selection = Some(*selected_entity),
                        }
                    },

                    // Deselection
                    SelectionEvent::JustDeselected(_) => (), //{
                    //     match select_target.body {
                    //         Some(existing) if existing == *deselected_entity => select_target.body = None,
                    //         Some(_) => (),
                    //         None => (),
                    //     }
                    // }
                }
            },
            PickingEvent::Hover(hover) => {
                match hover {
                    HoverEvent::JustEntered(hovered_entity) => {
                        match select_target.hover {
                            Some(existing) if existing != *hovered_entity => select_target.hover = Some(*hovered_entity),
                            Some(_) => (),
                            None => select_target.hover = Some(*hovered_entity),
                        }
                    },
                    HoverEvent::JustLeft(dehovered_entity) => {
                        match select_target.hover {
                            Some(existing) if existing == *dehovered_entity => select_target.hover = None,
                            Some(_) => (),
                            None => (),
                        }
                    },
                }
            },
        }
    }
}
