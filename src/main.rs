use bevy::{prelude::*, ecs::system::QuerySingleError};

#[derive(Component)]
struct Player {
    speed: f32,
}

struct TeleportClass {
    distance: f32,
}

#[derive(Component)]
enum Classes {
    Teleport(TeleportClass),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup)
        .add_system(movement_input_system)
        .add_system(ability_system)
        .run();
}

fn setup(
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
 
    let sphere = meshes.add(Mesh::from(shape::Icosphere{ subdivisions: 4, radius: 1.0 }));
    let player_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
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

fn ability_system(
    key_input: Res<Input<KeyCode>>,
    mut class_q: Query<(&Classes, &mut Transform)>,
){
    let (class, mut transform) = match class_q.get_single_mut(){
        Ok((class, transform)) => (class, transform),
        Err(v) => {
            match v{
                QuerySingleError::NoEntities(_) => return,
                QuerySingleError::MultipleEntities(_) => panic!("Found multiple entities with abilities"),
            }
        }
    };

    if key_input.pressed(KeyCode::LShift){
        match class {
            Classes::Teleport(teleport_class) => {
                if key_input.just_pressed(KeyCode::LShift) {
                    let mut dir = Vec2::new(0.0, 0.0);
                    if key_input.pressed(KeyCode::Left){
                        dir.x -= 1.0;
                    }
                    if key_input.pressed(KeyCode::Right){
                        dir.x += 1.0;
                    }
                    if key_input.pressed(KeyCode::Down){
                        dir.y -= 1.0;
                    }
                    if key_input.pressed(KeyCode::Up){
                        dir.y += 1.0;
                    }

                    dir = dir.normalize_or_zero() * teleport_class.distance;
                    transform.translation.x += dir.x;
                    transform.translation.y += dir.y;
                }
            },
        }
    }
}

fn movement_input_system(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Player, &mut Transform)>,
){
    let (player, mut transform) = player_q.single_mut();

    let dist = player.speed * time.delta_seconds();

    if key_input.pressed(KeyCode::Left){
        transform.translation.x -= dist;
    }
    if key_input.pressed(KeyCode::Right){
        transform.translation.x += dist;
    }
    if key_input.pressed(KeyCode::Down){
        transform.translation.y -= dist;
    }
    if key_input.pressed(KeyCode::Up){
        transform.translation.y += dist;
    }
}
