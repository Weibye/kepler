use std::f32::consts::PI;

use bevy::{math::Vec3, prelude::{Assets, Commands, Mesh, ResMut, info}};
use kepler::{Ellipse, OrbitalBody};

use crate::orbit::bundles::{OrbitalBodyBundle, TransformBundle};


pub fn create_ellipse(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,) {

    let ellipse = Ellipse::from_semi(10.0, 3.0);

    commands.spawn().insert(Ellipse::from_semi(10.0, 3.0));

    let nodes = 24;
    let angle = 2.0 * PI / nodes as f32;
    for n in 0..nodes{
        let n_angle = angle * n as f32;
        let body = OrbitalBody::from_sphere(0.2, 0.1, -0.3);
        let (x, y) = ellipse.perimeter_point(n_angle);
        let position = Vec3::new(x, 0.0, y);
        info!("Perimeter: {:?}, {:?}", x, y);
        
        commands
            .spawn()
            .insert_bundle(OrbitalBodyBundle::from_orbital_body(body, &mut meshes))
            .insert_bundle(TransformBundle::from_translation(position))
        ;
    }
    
}