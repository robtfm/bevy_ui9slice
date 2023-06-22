use bevy::prelude::*;
use bevy_ui9slice::{Ui9Slice, Ui9SlicePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Ui9SlicePlugin)
        .add_startup_system(setup)
        .add_system(size)
        .add_system(update)
        .run()
}

#[derive(Component)]
pub struct Marker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle::default());

    commands.spawn((
        NodeBundle::default(),
        Ui9Slice {
            image: asset_server.load("happy_9slice.png"),
            center_region: UiRect::all(Val::Px(20.0)),
        },
        Marker,
    ));
}

fn size(mut sz: Query<&mut Style, With<Marker>>, time: Res<Time>) {
    let Ok(mut s) = sz.get_single_mut() else { return; };
    let (x, y) = time.elapsed_seconds().sin_cos();
    s.size = Size::new(Val::Px(200.0 + x * 100.0), Val::Px(200.0 + y * 100.0));
}

fn update(
    key: Res<Input<KeyCode>>,
    mut slice: Query<&mut Ui9Slice>,
    asset_server: Res<AssetServer>,
) {
    if key.just_pressed(KeyCode::Q) {
        // hot swap slice margins
        // you can use % of image size if you like
        // undefined/auto will be treated as zero
        slice.single_mut().center_region = UiRect {
            top: Val::Percent(50.0),
            bottom: Val::Percent(0.0),
            left: Val::Percent(20.0),
            right: Val::Auto,
        };
    }
    if key.just_pressed(KeyCode::W) {
        // hot swap slice margins
        // you can use % of image size if you like
        // undefined/auto will be treated as zero
        slice.single_mut().center_region = UiRect {
            top: Val::Percent(20.0),
            bottom: Val::Percent(20.0),
            left: Val::Percent(20.0),
            right: Val::Percent(20.0),
        };
    }
    if key.just_pressed(KeyCode::E) {
        // hot swap the image
        // note using slices of 20 pixels on this image of only 10 pixels looks bad (press Q/W to use percent instead)
        slice.single_mut().image = asset_server.load("border.png");
    }
    if key.just_pressed(KeyCode::R) {
        // hot swap the image
        slice.single_mut().image = asset_server.load("happy_9slice.png");
    }
}
