use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct BloomPlugin;

#[derive(Component)]
pub enum BloomDirection {
    Bright,
    Dim,
}

impl Plugin for BloomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_bloom_settings);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 7.4, 7.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(50., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.25, 0.4, 0.1))),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(25., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1.25, 1.4, 1.1))),
        transform: Transform::from_translation(Vec3::new(50., 100., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(10., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1., 1., 1.))),
        transform: Transform::from_translation(Vec3::new(150., 150., 0.)),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 12.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut hex: Query<(Entity, &mut BloomDirection)>,
    mut text: Query<&mut Text>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    let (_, mut bloom_direction) = hex.single_mut();

    let bloom_max = 0.4;
    let bloom_min = 0.1;
    match bloom_settings {
        (_, Some(mut bloom_settings)) => {
            *text = "bloom intensity: ".to_string();
            text.push_str(&format!("{:.2}\n", bloom_settings.intensity));
            match *bloom_direction {
                BloomDirection::Bright => {
                    bloom_settings.intensity += 0.001;
                    if bloom_settings.intensity > bloom_max {
                        *bloom_direction = BloomDirection::Dim;
                    }
                },
                BloomDirection::Dim => {
                    bloom_settings.intensity -= 0.001;
                    if bloom_settings.intensity < bloom_min {
                        *bloom_direction = BloomDirection::Bright;
                    }
                },
            }
        }
        (_, None) => {
            *text = "Bloom: Off".to_string();
        }
    }
    //         text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
    //         text.push_str(&format!(
    //             "(W/S) Low-frequency boost: {}\n",
    //             bloom_settings.low_frequency_boost
    //         ));
    //         text.push_str(&format!(
    //             "(E/D) Low-frequency boost curvature: {}\n",
    //             bloom_settings.low_frequency_boost_curvature
    //         ));
    //         text.push_str(&format!(
    //             "(R/F) High-pass frequency: {}\n",
    //             bloom_settings.high_pass_frequency
    //         ));
    //         text.push_str(&format!(
    //             "(T/G) Mode: {}\n",
    //             match bloom_settings.composite_mode {
    //                 BloomCompositeMode::EnergyConserving => "Energy-conserving",
    //                 BloomCompositeMode::Additive => "Additive",
    //             }
    //         ));
    //         text.push_str(&format!(
    //             "(Y/H) Threshold: {}\n",
    //             bloom_settings.prefilter_settings.threshold
    //         ));
    //         text.push_str(&format!(
    //             "(U/J) Threshold softness: {}\n",
    //             bloom_settings.prefilter_settings.threshold_softness
    //         ));
    //
    //         if keycode.just_pressed(KeyCode::Space) {
    //             commands.entity(entity).remove::<BloomSettings>();
    //         }
    //
    //         let dt = time.delta_seconds();
    //
    //         if keycode.pressed(KeyCode::A) {
    //             bloom_settings.intensity -= dt / 10.0;
    //         }
    //         if keycode.pressed(KeyCode::Q) {
    //             bloom_settings.intensity += dt / 10.0;
    //         }
    //         bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);
    //
    //         if keycode.pressed(KeyCode::S) {
    //             bloom_settings.low_frequency_boost -= dt / 10.0;
    //         }
    //         if keycode.pressed(KeyCode::W) {
    //             bloom_settings.low_frequency_boost += dt / 10.0;
    //         }
    //         bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);
    //
    //         if keycode.pressed(KeyCode::D) {
    //             bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
    //         }
    //         if keycode.pressed(KeyCode::E) {
    //             bloom_settings.low_frequency_boost_curvature += dt / 10.0;
    //         }
    //         bloom_settings.low_frequency_boost_curvature =
    //             bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);
    //
    //         if keycode.pressed(KeyCode::F) {
    //             bloom_settings.high_pass_frequency -= dt / 10.0;
    //         }
    //         if keycode.pressed(KeyCode::R) {
    //             bloom_settings.high_pass_frequency += dt / 10.0;
    //         }
    //         bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);
    //
    //         if keycode.pressed(KeyCode::G) {
    //             bloom_settings.composite_mode = BloomCompositeMode::Additive;
    //         }
    //         if keycode.pressed(KeyCode::T) {
    //             bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
    //         }
    //
    //         if keycode.pressed(KeyCode::H) {
    //             bloom_settings.prefilter_settings.threshold -= dt;
    //         }
    //         if keycode.pressed(KeyCode::Y) {
    //             bloom_settings.prefilter_settings.threshold += dt;
    //         }
    //         bloom_settings.prefilter_settings.threshold =
    //             bloom_settings.prefilter_settings.threshold.max(0.0);
    //
    //         if keycode.pressed(KeyCode::J) {
    //             bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
    //         }
    //         if keycode.pressed(KeyCode::U) {
    //             bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
    //         }
    //         bloom_settings.prefilter_settings.threshold_softness = bloom_settings
    //             .prefilter_settings
    //             .threshold_softness
    //             .clamp(0.0, 1.0);
    //     }
    //
    //     (entity, None) => {
    //         *text = "Bloom: Off (Toggle: Space)".to_string();
    //
    //         if keycode.just_pressed(KeyCode::Space) {
    //             commands.entity(entity).insert(BloomSettings::default());
    //         }
    //     }
    // }
}
