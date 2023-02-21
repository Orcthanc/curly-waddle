use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

mod player;
mod basic_shapes;

use player::player::*;
use bevy::ecs::query::QuerySingleError;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup_default_scene)
        .add_system(movement_input_system)
        .add_system(ability_system)
        .add_system(follow_cam_system)
        .add_system(check_orb)
        .run();
}

#[derive(Component)]
struct Orb {}

fn check_orb(
    mut orb_q: Query<(&Orb, &mut Transform), Without<Player>>,
    mut player_q: Query<(&Transform, &mut Player), Without<Orb>>,
) {
    let (_, mut transform) = match orb_q.get_single_mut(){
        Ok((orb, transform)) => (orb, transform),
        Err(v) => {
            match v{
                QuerySingleError::NoEntities(_) => panic!("No orb found"),
                QuerySingleError::MultipleEntities(_) => panic!("Found multiple orbs"),
            }
        }
    };

    let (p_trans, mut player) = player_q.single_mut();

    if p_trans.translation.distance(transform.translation) < 3.0 {
        let angle: f32 = rand::thread_rng().gen::<f32>() * std::f32::consts::PI * 2.0;
        transform.translation += Vec3::new(40.0 * angle.cos(), 40.0 * angle.sin(), 0.0);
        player.invert = !player.invert;
    }
}

fn setup_default_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut cam = Camera3dBundle::default();

    //TODO make diagonal
    cam.transform.translation = Vec3::new( 0.0, 0.0, 80.0 );
    //TODO change up to 0 0 1
    cam.transform.look_at(Vec3::new( 0.0, 80.0, 0.0 ), Vec3::new( 0.0, 1.0, 0.0 ));

    commands.spawn(cam);
 
    //Player
    let sphere = meshes.add(Mesh::from(shape::Icosphere{ subdivisions: 4, radius: 1.0 }));
    let player_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.7, 0.7),
        ..Default::default()
    });

    let mut watch = Stopwatch::new();
    watch.set_elapsed(Duration::from_secs(5));

    commands.spawn(PbrBundle {
        mesh: sphere,
        material: player_mat,
        transform: Transform::from_xyz( 0.0, 0.0, 0.0 ),
        ..Default::default()
    })
    .insert(Player{ speed: 20.0, invert: true })
    .insert(Classes::Teleport(TeleportClass{ distance: 10.0 }))
    .insert(TeleportCooldown{ time: watch });
    //.insert(Classes::Sprint(SprintClass{ sprint_multiplier: 20.0 }));
 
    //Ground
    let square = meshes.add(Mesh::from(shape::Plane{ size: 1.0 }));
    let white_mat = materials.add( StandardMaterial{ 
        base_color: Color::rgb(1.0, 1.0, 1.0),
        reflectance: 0.1,
        ..Default::default()
    });

    let black_mat = materials.add( StandardMaterial{ 
        base_color: Color::rgb(0.0, 0.0, 0.0),
        reflectance: 0.1,
        ..Default::default()
    });

    let size = 24.0;

    for y in -10..10 {
        for x in -10..10 {
            let x_p = x as f32 * size * std::f32::consts::FRAC_1_SQRT_2;
            let y_p = y as f32 * size * std::f32::consts::FRAC_1_SQRT_2;
            commands.spawn(PbrBundle {
                mesh: square.to_owned(),
                material: if ((x + y) as i32).abs() % 2 == 1 { white_mat.to_owned() } else { black_mat.to_owned() },
                transform: Transform {
                    translation: Vec3::new(x_p + y_p, x_p - y_p, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4) * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                    scale: Vec3::new(size, size, size )},
                ..Default::default()
            });
        }
    }


    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { 
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(Mat4::look_at_rh(Vec3::new(-100.0, 100.0, 200.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0))),
        ..Default::default()
    });


    //Testing stuff
    let circle = meshes.add(basic_shapes::default_circle(64, 1.0));

    let hit_zone_mat = materials.add(StandardMaterial { 
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.8),
        reflectance: 0.1,
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: circle,
        material: hit_zone_mat.to_owned(),
        transform: Transform {
            translation: Vec3::new(-40.0, 0.0, 0.01),
            rotation: Quat::from_rotation_x(0.0),
            scale: Vec3::new(3.0, 3.0, 3.0),
        },
        ..Default::default()
    })
    .insert(Orb{});
}
