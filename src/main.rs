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
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        // Main Menu systems
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
        .add_systems(Update, menu_button_system.run_if(in_state(AppState::MainMenu)))
        // Model Pose systems
        .add_systems(OnEnter(AppState::ModelPose), setup_model_pose)
        .add_systems(OnExit(AppState::ModelPose), cleanup_model_pose)
        .add_systems(Update, (rotate_model, back_button_system).run_if(in_state(AppState::ModelPose)))
        // Memory Game systems
        .add_systems(OnEnter(AppState::MemoryGame), setup_memory_game)
        .add_systems(OnExit(AppState::MemoryGame), cleanup_memory_game)
        .add_systems(Update, back_button_system.run_if(in_state(AppState::MemoryGame)))
        // Rotation Game systems
        .add_systems(OnEnter(AppState::RotationGame), setup_rotation_game)
        .add_systems(OnExit(AppState::RotationGame), cleanup_rotation_game)
        .add_systems(Update, back_button_system.run_if(in_state(AppState::RotationGame)))
        .run();
}

/// App states for different modes/exercises
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    ModelPose,
    MemoryGame,
    RotationGame,
}

/// Marker component for UI elements that should be cleaned up
#[derive(Component)]
struct StateCleanup;

/// Marker component for the spinning model
#[derive(Component)]
struct SpinningModel;

/// Marker component for menu buttons
#[derive(Component)]
enum MenuButton {
    ModelPose,
    MemoryGame,
    RotationGame,
}

/// Marker component for back button
#[derive(Component)]
struct BackButton;

/// Setup the camera once at startup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Setup the main menu UI
fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            StateCleanup,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Perspecta"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Learning to Draw"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(80.0)),
                    ..default()
                },
            ));

            // 3D Model Pose button
            spawn_menu_button(parent, "3D Model Pose", MenuButton::ModelPose);

            // Memory Game button
            spawn_menu_button(parent, "Memory Game", MenuButton::MemoryGame);

            // Rotation Game button
            spawn_menu_button(parent, "Rotation Game", MenuButton::RotationGame);
        });
}

/// Helper function to spawn a menu button
fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, button_type: MenuButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Helper function to spawn a back button
fn spawn_back_button(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                ..default()
            },
            StateCleanup,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    BackButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("← Back"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// System to handle menu button interactions
fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.35, 0.75, 0.35).into();
                match menu_button {
                    MenuButton::ModelPose => next_state.set(AppState::ModelPose),
                    MenuButton::MemoryGame => next_state.set(AppState::MemoryGame),
                    MenuButton::RotationGame => next_state.set(AppState::RotationGame),
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.25, 0.25, 0.25).into();
            }
        }
    }
}

/// System to handle back button interactions
fn back_button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.35, 0.75, 0.35).into();
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.25, 0.25, 0.25).into();
            }
        }
    }
}

/// Cleanup menu UI
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// MODEL POSE MODE
// ============================================================================

/// Setup the 3D model pose scene
fn setup_model_pose(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add back button
    spawn_back_button(&mut commands);

    // Remove 2D camera and add 3D camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        StateCleanup,
    ));

    // Load and spawn the human model
    commands.spawn((
        SceneRoot(asset_server.load("models/cesium_man.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningModel,
        StateCleanup,
    ));

    // Add a key light (main light source)
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        StateCleanup,
    ));

    // Add a fill light (softer, from the other side)
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 3.0),
        StateCleanup,
    ));
}

/// Rotate the model continuously
fn rotate_model(time: Res<Time>, mut query: Query<&mut Transform, With<SpinningModel>>) {
    for mut transform in &mut query {
        // Rotate around the Y axis for a slow turnaround
        transform.rotate_y(time.delta_secs() * 0.8);
    }
}

/// Cleanup model pose scene
fn cleanup_model_pose(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// MEMORY GAME MODE
// ============================================================================

/// Setup the memory game scene
fn setup_memory_game(mut commands: Commands) {
    spawn_back_button(&mut commands);

    // Placeholder text
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.15, 0.25)),
            StateCleanup,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Memory Game\n\n(Coming Soon)"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Cleanup memory game scene
fn cleanup_memory_game(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// ROTATION GAME MODE
// ============================================================================

/// Setup the rotation game scene
fn setup_rotation_game(mut commands: Commands) {
    spawn_back_button(&mut commands);

    // Placeholder text
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.2, 0.25)),
            StateCleanup,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Rotation Game\n\n(Coming Soon)"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Cleanup rotation game scene
fn cleanup_rotation_game(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
