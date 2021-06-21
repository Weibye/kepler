use std::f32::consts::PI;

use bevy::{math::{Quat, Vec3}, prelude::{Assets, BuildChildren, Commands, GlobalTransform, Mesh, ResMut, Transform, info}};
use kepler::{Ellipse, EllipticalOrbitBundle, OrbitalBody, TransformBundle};
use rand::Rng;

use crate::orbit::bundles::{OrbitalBodyBundle, ReferenceFrameBundle};


pub fn create_ellipse(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,) {

    let ellipse = Ellipse::from_semi(10.0, 3.0);

    commands.spawn().insert(Ellipse::from_semi(10.0, 3.0));
    commands.spawn().insert(Ellipse::from_semi(8.0, 6.0));
    commands.spawn().insert(Ellipse::from_semi(5.0, 4.89));

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

pub fn create_ellipse_bundles(mut commands: Commands) {

    let mut rng = rand::thread_rng();

    // let ellipse = Ellipse::from_semi(10.0, 3.0);
    let reference_one_local = Transform::default();
    let reference_one_global = GlobalTransform::default();

    let pos_two = Vec3::new(0.0, 10.0, 0.0);
    let rot_two = Quat::from_rotation_ypr(
        rng.gen_range(0.0..(2.0 * PI)),
        rng.gen_range(0.0..(2.0 * PI)),
        rng.gen_range(0.0..(2.0 * PI)),
    );
    let reference_two_local = Transform {
        translation: pos_two,
        scale: Vec3::ONE,
        rotation: rot_two,
    };
    let reference_two_global = GlobalTransform {
        translation: pos_two,
        scale: Vec3::ONE,
        rotation: rot_two
    };

    let pos_three = Vec3::new(0.0, -10.0, 0.0);
    let rot_three = Quat::from_rotation_ypr(
        rng.gen_range(0.0..(2.0 * PI)),
        rng.gen_range(0.0..(2.0 * PI)),
        rng.gen_range(0.0..(2.0 * PI)),
    );
    let reference_three_local = Transform {
        translation: pos_three,
        scale: Vec3::ONE,
        rotation: rot_three
    };
    let reference_three_global = GlobalTransform {
        translation: pos_three,
        scale: Vec3::ONE,
        rotation: rot_three
    };

    let parent_one = commands.spawn_bundle(ReferenceFrameBundle::from_transforms(reference_one_local, reference_one_global)).id();
    let parent_two = commands.spawn_bundle(ReferenceFrameBundle::from_transforms(reference_two_local, reference_two_global)).id();
    let parent_three = commands.spawn_bundle(ReferenceFrameBundle::from_transforms(reference_three_local, reference_three_global)).id();

    let child_one = commands.spawn_bundle(EllipticalOrbitBundle::new(
        5.0, 
        0.8,
        0.2,
        0.2,
        0.0))
        .id()
    ;

    let child_two = commands.spawn_bundle(EllipticalOrbitBundle::new(
        5.0, 
        0.8,
        0.2,
        0.2,
        0.0))
        .id()
    ;

    let child_three = commands.spawn_bundle(EllipticalOrbitBundle::new(
        5.0, 
        0.8,
        0.2,
        0.2,
        0.0))
        .id()
    ;

    commands.entity(parent_one).push_children(&[child_one]);
    commands.entity(parent_two).push_children(&[child_two]);
    commands.entity(parent_three).push_children(&[child_three]);
    
    // let nodes = 1;
    // let angle = 2.0 * PI / nodes as f32;
    // for n in 0..nodes{
    //     let n_angle = angle * n as f32;
    //     let body = OrbitalBody::from_sphere(0.2, 0.1, -0.3);
    //     let (x, y) = ellipse.perimeter_point(n_angle);
    //     let position = Vec3::new(x, 0.0, y);
    //     info!("Perimeter: {:?}, {:?}", x, y);
        
    //     commands
    //         .spawn()
    //         .insert_bundle(OrbitalBodyBundle::from_orbital_body(body, &mut meshes))
    //         .insert_bundle(TransformBundle::from_translation(position))
    //     ;
    // }
}

