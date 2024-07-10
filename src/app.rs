use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
    }
    app.add_systems(Startup, (add_camera, add_player));
    app.add_systems(Update, respond_to_keyboard);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default()
    );
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn get_camera_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_camera_rotation(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.rotation.z
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn count_n_cameras(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Camera>();
    query.iter(app.world()).len()
}

fn respond_to_keyboard(
    mut query: Query<(&mut Transform, &Camera)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, _) = query.single_mut();
    use bevy::input::keyboard::KeyCode;
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.rotate_z(-0.1);
    }
    if input.pressed(KeyCode::KeyE) {
        transform.rotate_z(0.1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_moving_cameras() {
        let mut app = App::new();
        assert_eq!(count_n_cameras(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_moving_camera() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_cameras(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_camera_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_camera_is_not_rotated_at_start() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_moves_when_pressed_up() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_right() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_down() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowDown);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_left() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);
        app.update();assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }

    #[test]
    fn test_camera_rotates_when_pressed_q() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyQ);
        app.update();

        println!("{:?}", get_camera_rotation(&mut app));

        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_rotates_when_pressed_e() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();

        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

}
