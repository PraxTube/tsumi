use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 128.0, tile_size_y = 128.0, columns = 8, rows = 3))]
    #[asset(path = "player.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(
        paths(
            "player.trickfilm#idle",
            "player.trickfilm#walk",
            "player.trickfilm#run",
        ),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,
}
