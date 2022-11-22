use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(PhysicsHooksWithQueryResource(Box::new(MyPhysicsHooks {})))
        .add_startup_system(setup)
        .add_system(movement)
        // .add_system(contact_modifier)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
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
    let player_radius = 35.0;
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(player_radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(player_start_pos.extend(1.0)),
            ..Default::default()
        })
        .insert(Collider::ball(player_radius))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce::default())
        .insert(Restitution::coefficient(1.0))
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 0.1,
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(VirtualDPad::arrow_keys(), Action::Move)
                .insert(VirtualDPad::wasd(), Action::Move)
                .set_gamepad(Gamepad { id: 0 })
                .build(),
        })
        .insert(Player)
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS);

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
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(location.extend(0.0)),
            ..Default::default()
        },
        Collider::ball(radius),
        ContactForceEventThreshold(0.0),
        RigidBody::Fixed,
        Restitution::coefficient(3.0),
        ActiveHooks::MODIFY_SOLVER_CONTACTS,
    ));
}

const MOVE_FORCE: f32 = 1500.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in query.iter_mut() {
        let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}

struct MyPhysicsHooks;

impl PhysicsHooksWithQuery<NoUserData> for MyPhysicsHooks {
    fn modify_solver_contacts(
        &self,
        context: ContactModificationContextView,
        _user_data: &Query<NoUserData>,
    ) {
        for solver_contact in &mut *context.raw.solver_contacts {
            *solver_contact.tangent_velocity = // ???
        }
    }
}
