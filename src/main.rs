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
        .add_systems(Update, rotate_quad)
        .run();
}

/// Marker component for the spinning quad
#[derive(Component)]
struct SpinningQuad;

/// Setup the 3D scene with a spinning quad
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a quad (plane) that will spin
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.8),
            metallic: 0.5,
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningQuad,
    ));

    // Add a light source
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Add a camera looking at the quad
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Rotate the quad continuously
fn rotate_quad(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpinningQuad>>,
) {
    for mut transform in &mut query {
        // Rotate around the Y axis
        transform.rotate_y(time.delta_secs() * 1.5);
        // Also rotate a bit around the X axis for a more interesting spin
        transform.rotate_x(time.delta_secs() * 0.5);
    }
}
