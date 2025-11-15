use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Perspecta - Learning to Draw".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_model)
        .run();
}

/// Marker component for the spinning model
#[derive(Component)]
struct SpinningModel;

/// Setup the 3D scene with a human model
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load and spawn the human model
    commands.spawn((
        SceneRoot(asset_server.load("models/cesium_man.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningModel,
    ));

    // Add a key light (main light source)
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Add a fill light (softer, from the other side)
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 3.0),
    ));

    // Add a camera positioned to view the human model
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

/// Rotate the model continuously
fn rotate_model(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpinningModel>>,
) {
    for mut transform in &mut query {
        // Rotate around the Y axis for a slow turnaround
        transform.rotate_y(time.delta_secs() * 0.8);
    }
}
