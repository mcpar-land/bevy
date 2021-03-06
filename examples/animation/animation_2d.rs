use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/icon.png");

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(AnimationSplineTransform {
            translation_x: Spline::from_vec(vec![
                Key::new(0.0, 0.0, Interpolation::Cosine),
                Key::new(1.0, -150.0, Interpolation::Cosine),
                Key::new(2.0, 100.0, Interpolation::Cosine),
                Key::new(3.0, 0.0, Interpolation::Cosine),
            ]),
            translation_y: Spline::from_vec(vec![
                Key::new(0.0, 100.0, Interpolation::Linear),
                Key::new(1.5, -100.0, Interpolation::Linear),
                Key::new(3.0, 100.0, Interpolation::Linear),
            ]),
            translation_z: Spline::from_vec(vec![]),
            rotation: Spline::from_vec(vec![
                Key::new(
                    0.0,
                    Quat::from_rotation_ypr(0., 0., 720_f32.to_radians()),
                    Interpolation::Linear,
                ),
                Key::new(
                    1.0,
                    Quat::from_rotation_ypr(0., 0., 480_f32.to_radians()),
                    Interpolation::Linear,
                ),
                Key::new(
                    2.0,
                    Quat::from_rotation_ypr(0., 0., 240_f32.to_radians()),
                    Interpolation::Linear,
                ),
                Key::new(
                    3.0,
                    Quat::from_rotation_ypr(0., 0., 0_f32.to_radians()),
                    Interpolation::Linear,
                ),
            ])
            .slerp(),
            scale: Spline::from_vec(vec![
                Key::new(0.0, 1.0, Interpolation::Cosine),
                Key::new(0.5, 1.5, Interpolation::Cosine),
                Key::new(1.0, 1.0, Interpolation::Cosine),
            ]),
            loop_style: LoopStyle::PingPong,
            ..Default::default()
        });
}
