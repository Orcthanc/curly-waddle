pub mod player{
    use bevy::{prelude::*, ecs::system::QuerySingleError};

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
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

    pub fn movement_input_system(
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
}
