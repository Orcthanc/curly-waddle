pub mod player{
    use bevy::{prelude::*, ecs::query::QuerySingleError, time::Stopwatch};

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
        pub invert: bool,
    }

    pub struct TeleportClass {
        pub distance: f32,
    }

    #[derive(Component)]
    pub struct TeleportCooldown{
        pub time: Stopwatch,
    }

    #[derive(Component)]
    pub enum Classes {
        Teleport(TeleportClass),
    }

    fn project_cursor(
        window: &Window,
        cursor_position: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Vec3 {
        let w_size = Vec2::new(window.width(), window.height());
        let ndc = (cursor_position / w_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos1 = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos2 = camera_transform.translation();

        let res = world_pos1 - (world_pos1 - world_pos2) * (world_pos1.z / (world_pos1 - world_pos2).z);
        
        return res;
    }

    pub fn ability_system(
        key_input: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut class_q: Query<(&Classes, &Player, &mut TeleportCooldown, &mut Transform)>,
        windows: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform)>
    ){
        let (class, player, mut cooldown, mut transform) = match class_q.get_single_mut(){
            Ok((class, player, cooldown, transform)) => (class, player, cooldown, transform),
            Err(v) => {
                match v{
                    QuerySingleError::NoEntities(_) => return,
                    QuerySingleError::MultipleEntities(_) => panic!("Found multiple entities with abilities"),
                }
            }
        };
        cooldown.time.tick(time.delta());

        if key_input.just_pressed(KeyCode::Space){
            match class {
                Classes::Teleport(teleport_class) => {

                    if cooldown.time.elapsed_secs() > 5.0 {

                        cooldown.time.reset();

                        let mut dir = Vec2::new(0.0, 0.0);

                        let (camera, camera_transform) = q_camera.single();
                        let window = windows.get_primary().unwrap();

                        if let Some(pos) = window.cursor_position() {
                            dir = project_cursor(window, pos, camera, camera_transform).truncate() - transform.translation.truncate();
                        }

                        dir = dir.normalize_or_zero() * teleport_class.distance;

                        if player.invert {
                            dir *= -1.0;
                        }

                        transform.translation.x += dir.x;
                        transform.translation.y += dir.y;
                    }
                },
            }
        }
    }

    pub fn movement_input_system(
        time: Res<Time>,
        mouse_input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut player_q: Query<(&Player, &mut Transform)>,
        q_camera: Query<(&Camera, &GlobalTransform)>
    ){
        let (player, mut transform) = player_q.single_mut();

        let (camera, camera_transform) = q_camera.single();

        let dist = player.speed * time.delta_seconds();
        let window = windows.get_primary().unwrap();

        let mut dir = Vec2::new(0.0, 0.0);

        if mouse_input.pressed(MouseButton::Right) {
            if let Some(pos) = window.cursor_position() {
                dir = project_cursor(window, pos, camera, camera_transform).truncate() - transform.translation.truncate();
            }
        }

        dir = dir.normalize_or_zero() * dist;

        if player.invert {
            dir *= -1.0;
        }

        transform.translation.x += dir.x;
        transform.translation.y += dir.y;
    }

    pub fn follow_cam_system(
        time: Res<Time>,
        windows: Res<Windows>,
        player_q: Query<(&Player, &Transform)>,
        mut cam_q: Query<(&Camera, &GlobalTransform, &mut Transform), Without<Player>>,
    ){
        let (player, p_trans) = player_q.single();
        let (cam, c_trans, mut l_trans) = cam_q.single_mut();

        let window = windows.get_primary().unwrap();

        let cam_center = project_cursor(window, Vec2::new(window.width() / 2.0, window.height() / 2.0), cam, &c_trans);

        let mut dist = cam_center - p_trans.translation;
        let temp = dist * player.speed * time.delta_seconds();
        dist = if dist.length_squared() > temp.length_squared() { temp } else { dist };

        l_trans.translation -= dist;
    }
}
