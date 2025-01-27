fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //let rounded_map_size = (MAX_BLOCK_HEIGHT as f32).sqrt().ceil() as u32;
    info!("spawning map size: {} by {}", MAP_LENGTH, MAP_LENGTH);
    let map_size = TilemapSize {
        x: MAP_LENGTH + 2,
        y: MAP_LENGTH + 2,
    };
    let tile_storage = TileStorage::empty(map_size);

    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().id();
    let texture_handle: Handle<Image> =
        asset_server.load("spritesheet/ss-land-v12-gimp-64-spaced.png");

    let center = get_tilemap_center_transform(&map_size, &TILE_SIZE.into(), &map_type, 0.0);
    // need to do an offset so it lines up with the chunking logic overlay sprites and tiles. Right now it's off by half the distance of a tile in both x/y directions
    // given a 66 pixel tile, the offset would be +33., +33. in for x/y.
    let offset_tran = Vec3::new(
        center.translation.x + (TILE_SIZE.x / 2.),
        center.translation.y + (TILE_SIZE.y / 2.),
        0.,
    );
    let transform_for_map = Transform::from_translation(offset_tran);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: TILE_SIZE.x,
            y: TILE_SIZE.y,
        },
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        spacing: TILE_SPACING,
        transform: transform_for_map,
        ..Default::default()
    });

    //    texture: &Handle<Image>,
    //     layout: &Handle<TextureAtlasLayout>,

    //     texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetBuilding {
        layout: building_texture_atlas,
        texture: building_texture,
    });
}