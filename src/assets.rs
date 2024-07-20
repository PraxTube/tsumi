use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // --- PLAYER ---
    #[asset(path = "player.png")]
    pub player_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 3))]
    pub player_layout: Handle<TextureAtlasLayout>,

    #[asset(
        paths(
            "player.trickfilm#idle",
            "player.trickfilm#walk",
            "player.trickfilm#run",
        ),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    // --- MISC ---
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,
}
