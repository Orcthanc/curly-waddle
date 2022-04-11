use bevy::prelude::*;

mod player;

use player::player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup_default_scene)
        .add_system(movement_input_system)
        .add_system(ability_system)
        .add_system(follow_cam_system)
        .run();
}

fn setup_default_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut cam = PerspectiveCameraBundle::new_3d();

    //TODO make diagonal
    cam.transform.translation = Vec3::new( 0.0, 0.0, 80.0 );
    //TODO change up to 0 0 1
    cam.transform.look_at(Vec3::new( 0.0, 0.0, 0.0 ), Vec3::new( 0.0, 1.0, 0.0 ));

    info!("{:?}", cam.perspective_projection);

    commands.spawn_bundle(cam);
    commands.spawn_bundle(UiCameraBundle::default());
 
    //Player
    let sphere = meshes.add(Mesh::from(shape::Icosphere{ subdivisions: 4, radius: 1.0 }));
    let player_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.7, 0.7),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: sphere,
        material: player_mat,
        transform: Transform::from_xyz( 0.0, 0.0, 0.0 ),
        ..Default::default()
    })
    .insert(Player{ speed: 20.0 })
    .insert(Classes::Teleport(TeleportClass{ distance: 10.0 }));
    //.insert(Classes::Sprint(SprintClass{ sprint_multiplier: 20.0 }));
 
    //Ground
    let square = meshes.add(Mesh::from(shape::Plane{ size: 1.0 }));
    let white_mat = materials.add( StandardMaterial{ 
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    let black_mat = materials.add( StandardMaterial{ 
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let size = 32.0;

    for y in -10..10 {
        for x in -10..10 {
            commands.spawn_bundle(PbrBundle {
                mesh: square.to_owned(),
                material: if ((x + y) as i32).abs() % 2 == 1 { white_mat.to_owned() } else { black_mat.to_owned() },
                transform: Transform {
                    translation: Vec3::new(x as f32 * size, y as f32 * size, 0.0),
                    rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                    scale: Vec3::new(size, size, size )},
                ..Default::default()
            });
        }
    }


    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight { 
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 1000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(Mat4::face_toward(Vec3::new(-100.0, 100.0, 200.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0))),
        ..Default::default()
    });
}
