pub mod player{
    use bevy::{prelude::*, ecs::system::QuerySingleError, render::camera::CameraPlugin};

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
    }

    pub struct TeleportClass {
        pub distance: f32,
    }

    pub struct SprintClass {
        pub sprint_multiplier: f32,
    }

    #[derive(Component)]
    pub enum Classes {
        Teleport(TeleportClass),
        Sprint(SprintClass),
    }

    pub fn ability_system(
        key_input: Res<Input<KeyCode>>,
        time: Res<Time>,
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
                        if key_input.pressed(KeyCode::A){
                            dir.x -= 1.0;
                        }
                        if key_input.pressed(KeyCode::D){
                            dir.x += 1.0;
                        }
                        if key_input.pressed(KeyCode::S){
                            dir.y -= 1.0;
                        }
                        if key_input.pressed(KeyCode::W){
                            dir.y += 1.0;
                        }

                        dir = dir.normalize_or_zero() * teleport_class.distance;
                        transform.translation.x += dir.x;
                        transform.translation.y += dir.y;
                    }
                },
                Classes::Sprint(sprint_class) => {
                    //TODO improve
                    let dist = sprint_class.sprint_multiplier * time.delta_seconds();

                    if key_input.pressed(KeyCode::A){
                        transform.translation.x -= dist;
                    }
                    if key_input.pressed(KeyCode::D){
                        transform.translation.x += dist;
                    }
                    if key_input.pressed(KeyCode::S){
                        transform.translation.y -= dist;
                    }
                    if key_input.pressed(KeyCode::W){
                        transform.translation.y += dist;
                    }
                }
            }
        }
    }

    pub fn movement_input_system(
        time: Res<Time>,
        key_input: Res<Input<KeyCode>>,
        mut player_q: Query<(&Player, &mut Transform)>,
    ){
        let (player, mut transform) = player_q.single_mut();

        let dist = player.speed * time.delta_seconds();

        if key_input.pressed(KeyCode::A){
            transform.translation.x -= dist;
        }
        if key_input.pressed(KeyCode::D){
            transform.translation.x += dist;
        }
        if key_input.pressed(KeyCode::S){
            transform.translation.y -= dist;
        }
        if key_input.pressed(KeyCode::W){
            transform.translation.y += dist;
        }
    }

    pub fn follow_cam_system(
        player_q: Query<(&Player, &Transform)>,
        mut cam_q: Query<(&Camera, &mut Transform), Without<Player>>,
    ){

        let (_, p_trans) = player_q.single();

        for (camera, mut c_trans) in cam_q.iter_mut(){
            if camera.name == Some(CameraPlugin::CAMERA_3D.to_string()){
                c_trans.translation.x = p_trans.translation.x;
                c_trans.translation.y = p_trans.translation.y;
            }
        }
    }
}
