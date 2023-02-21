pub mod player{
    use bevy::{prelude::*, ecs::query::QuerySingleError};

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
        pub invert: bool,
    }

    pub struct TeleportClass {
        pub distance: f32,
    }

    #[derive(Component)]
    pub enum Classes {
        Teleport(TeleportClass),
    }

    pub fn ability_system(
        key_input: Res<Input<KeyCode>>,
        //time: Res<Time>,
        mut class_q: Query<(&Classes, &Player, &mut Transform)>,
        windows: Res<Windows>,
    ){
        let (class, player, mut transform) = match class_q.get_single_mut(){
            Ok((class, player, transform)) => (class, player, transform),
            Err(v) => {
                match v{
                    QuerySingleError::NoEntities(_) => return,
                    QuerySingleError::MultipleEntities(_) => panic!("Found multiple entities with abilities"),
                }
            }
        };

        if key_input.just_pressed(KeyCode::Space){
            match class {
                Classes::Teleport(teleport_class) => {
                    let mut dir = Vec2::new(0.0, 0.0);

                    let window = windows.get_primary().unwrap();

                    if let Some(pos) = window.cursor_position() {
                        dir = pos - Vec2::new(window.width(), window.height()) / 2.0;
                    }

                    dir = dir.normalize_or_zero() * teleport_class.distance;

                    if player.invert {
                        dir *= -1.0;
                    }

                    transform.translation.x += dir.x;
                    transform.translation.y += dir.y;
                },
            }
        }
    }

    pub fn movement_input_system(
        time: Res<Time>,
        mouse_input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut player_q: Query<(&Player, &mut Transform)>,
    ){
        let (player, mut transform) = player_q.single_mut();

        let dist = player.speed * time.delta_seconds();
        let window = windows.get_primary().unwrap();

        let mut dir = Vec2::new(0.0, 0.0);

        if mouse_input.pressed(MouseButton::Right) {
            if let Some(pos) = window.cursor_position() {
                dir = pos - Vec2::new(window.width(), window.height()) / 2.0;
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
        player_q: Query<(&Player, &Transform)>,
        mut cam_q: Query<(&Camera, &mut Transform), Without<Player>>,
    ){

        let (_, p_trans) = player_q.single();

        for (_, mut c_trans) in cam_q.iter_mut(){
            c_trans.translation.x = p_trans.translation.x;
            c_trans.translation.y = p_trans.translation.y;
        }
    }
}
