use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Player circle
    let player_start_pos = Vec2::new(0.0, 0.0);
    let goal_start_pos = Vec2::new(0.0, 0.0);
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(35.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..Default::default()
    });

    spawn_random_bumper_circles(
        &mut commands,
        &mut meshes,
        &mut materials,
        vec![player_start_pos, goal_start_pos],
    );
}

fn spawn_random_bumper_circles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut places_to_avoid: Vec<Vec2>,
) {
    let mut rng = rand::thread_rng();
    let bumper_radius = 45.;
    let bumper_spacing = bumper_radius * 2.05;
    for _ in 0..16 {
        let mut pos = places_to_avoid[0];
        while places_to_avoid
            .iter()
            .map(|x| pos.distance(*x))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(500.)
            .partial_cmp(&bumper_spacing)
            .unwrap_or(std::cmp::Ordering::Equal)
            == std::cmp::Ordering::Less
        {
            pos = Vec2::new(rng.gen_range(-500.0..500.0), rng.gen_range(-300.0..300.0));
        }
        places_to_avoid.push(pos);
        spawn_bumper_circle(pos, bumper_radius, commands, meshes, materials);
    }
}

fn spawn_bumper_circle(
    location: Vec2,
    radius: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Player circle
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(radius).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(location.extend(0.0)),
        ..Default::default()
    });
}
