use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // --- CHARACTERS ---
    #[asset(path = "characters/ami.png")]
    pub ami_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 3))]
    pub ami_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "characters/ima.png")]
    pub ima_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 1))]
    pub ima_layout: Handle<TextureAtlasLayout>,

    #[asset(
        paths(
            "characters/character.trickfilm#idle",
            "characters/character.trickfilm#walk",
            "characters/character.trickfilm#run",
        ),
        collection(typed)
    )]
    pub character_animations: Vec<Handle<AnimationClip2D>>,

    // --- ASPECTS ---
    #[asset(path = "aspects/transparent_icon.png")]
    pub transparent_icon: Handle<Image>,
    #[asset(path = "aspects/placeholder_icon.png")]
    pub placeholder_icon: Handle<Image>,
    #[asset(path = "aspects/joy_icon.png")]
    pub joy_icon: Handle<Image>,
    #[asset(path = "aspects/sadness_icon.png")]
    pub sadness_icon: Handle<Image>,
    #[asset(path = "aspects/anger_icon.png")]
    pub anger_icon: Handle<Image>,
    #[asset(path = "aspects/fear_icon.png")]
    pub fear_icon: Handle<Image>,
    #[asset(path = "aspects/nostalgia_icon.png")]
    pub nostalgia_icon: Handle<Image>,
    #[asset(path = "aspects/motivation_icon.png")]
    pub motivation_icon: Handle<Image>,
    #[asset(path = "aspects/melancholy_icon.png")]
    pub melancholy_icon: Handle<Image>,
    #[asset(path = "aspects/hatred_icon.png")]
    pub hatred_icon: Handle<Image>,
    #[asset(path = "aspects/vengefulness_icon.png")]
    pub vengefulness_icon: Handle<Image>,
    #[asset(path = "aspects/elation_icon.png")]
    pub elation_icon: Handle<Image>,
    #[asset(path = "aspects/anticipation_icon.png")]
    pub anticipation_icon: Handle<Image>,
    #[asset(path = "aspects/envy_icon.png")]
    pub envy_icon: Handle<Image>,
    #[asset(path = "aspects/pride_icon.png")]
    pub pride_icon: Handle<Image>,
    #[asset(path = "aspects/forgiveness_icon.png")]
    pub forgiveness_icon: Handle<Image>,

    // --- EFFECTS ---
    #[asset(path = "effects/smoke.png")]
    pub smoke_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 12, rows = 1))]
    pub smoke_layout: Handle<TextureAtlasLayout>,

    #[asset(paths("effects/smoke.trickfilm#main",), collection(typed))]
    pub smoke_animations: Vec<Handle<AnimationClip2D>>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    #[asset(path = "map/tutorial_switch.png")]
    pub tutorial_switch_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 3, rows = 1))]
    pub tutorial_switch_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "map/tutorial_wall.png")]
    pub tutorial_wall: Handle<Image>,

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

    #[asset(path = "ui/keys/interact_key.png")]
    pub ui_interact_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 2, rows = 1))]
    pub ui_interact_key_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "ui/keys/down_key.png")]
    pub ui_down_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_down_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/up_key.png")]
    pub ui_up_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_up_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/left_key.png")]
    pub ui_left_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_left_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/right_key.png")]
    pub ui_right_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_right_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/shift_key.png")]
    pub ui_shift_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 66, tile_size_y = 34, columns = 2, rows = 1))]
    pub ui_shift_key_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "ui/keys/keys.trickfilm#key",
            "ui/keys/keys.trickfilm#arrow",
            "ui/keys/keys.trickfilm#shift"
        ),
        collection(typed)
    )]
    pub ui_keys_animations: Vec<Handle<AnimationClip2D>>,

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

    #[asset(path = "ui/vignette.png")]
    pub vignette: Handle<Image>,

    // --- AUDIO ---
    #[asset(path = "audio/ami_blip.ogg")]
    pub ami_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/ima_blip.ogg")]
    pub ima_blip_sound: Handle<AudioSource>,

    #[asset(path = "audio/koto.ogg")]
    pub koto_hit_sound: Handle<AudioSource>,
    #[asset(path = "audio/footstep.ogg")]
    pub footstep: Handle<AudioSource>,
    #[asset(path = "audio/select_aspect.ogg")]
    pub select_aspect: Handle<AudioSource>,
    #[asset(path = "audio/deselect_aspect.ogg")]
    pub deselect_aspect: Handle<AudioSource>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
    #[asset(path = "fonts/Silver.ttf")]
    pub silver_font: Handle<Font>,
}
