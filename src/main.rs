use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[cfg(feature = "dev")]
use iyes_perf_ui::{prelude::*, entries::PerfUiBundle};
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "dev")]
use bevy_blendy_cameras::{BlendyCamerasPlugin, OrbitCameraController};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        PerfUiPlugin,
        WorldInspectorPlugin::default(),
        BlendyCamerasPlugin,
    ))
    .add_systems(Startup, debug_setup);

    app.run()
}

// 生成调试用的相机和显示运行信息
#[cfg(feature = "dev")]
fn debug_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3., 3., 3.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCameraController::default(),
    ));

    commands.spawn(PerfUiBundle::default());
}