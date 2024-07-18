use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 112.0, columns = 1, rows = 1))]
    #[asset(path = "player.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(paths("player.trickfilm#idle",), collection(typed))]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,
}
