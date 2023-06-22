simple 9-slice scaling for ui images. 

- simple to use, slices are specified by a single UiRect
- immediately reactive to changes in image / slices / container size, no image manipulation / component management required
- slices can be specified as percent of the image size or directly as pixels

basic setup: 
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Ui9SlicePlugin)
        .add_startup_system(setup)
        .run()
}

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
```