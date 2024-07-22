use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;
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

    // --- ASPECTS ---
    #[asset(path = "aspects/placeholder_icon.png")]
    pub placeholder_icon: Handle<Image>,
    #[asset(path = "aspects/test_icon.png")]
    pub test_icon: Handle<Image>,
    #[asset(path = "aspects/joy_icon.png")]
    pub joy_icon: Handle<Image>,
    #[asset(path = "aspects/anger_icon.png")]
    pub anger_icon: Handle<Image>,
    #[asset(path = "aspects/nostalgia_icon.png")]
    pub nostalgia_icon: Handle<Image>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    #[asset(path = "map/bed.png")]
    pub bed_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub bed_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "map/aspect_socket_left.png")]
    pub aspect_socket_texture_left: Handle<Image>,
    #[asset(path = "map/aspect_socket_right.png")]
    pub aspect_socket_texture_right: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub aspect_socket_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "map/aspect_combiner.png")]
    pub aspect_combiner_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub aspect_combiner_layout: Handle<TextureAtlasLayout>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,

    #[asset(path = "ui/dialogue_edge.png")]
    pub dialogue_edge: Handle<Image>,
    #[asset(path = "ui/dialogue_continue.png")]
    pub dialogue_continue: Handle<Image>,

    #[asset(path = "ui/ami_character_icon.png")]
    pub ami_character_icon: Handle<Image>,
    #[asset(path = "ui/ima_character_icon.png")]
    pub ima_character_icon: Handle<Image>,

    // --- AUDIO ---
    #[asset(path = "audio/ami_blip.ogg")]
    pub ami_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/ima_blip.ogg")]
    pub ima_blip_sound: Handle<AudioSource>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
    #[asset(path = "fonts/Silver.ttf")]
    pub silver_font: Handle<Font>,

    // --- MISC ---
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,
}
