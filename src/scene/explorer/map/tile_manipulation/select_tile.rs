use bevy::prelude::*;

use crate::scene::{
    explorer::{
        ecs::{component::SelectedTile, hard::TILE_SIZE, resource::SpriteSheetManualSelectRes},
        input::hard::{AddTileManualSelectionSprite, RemoveTileManualSelectionSprite},
        map::ecs::component::SelectionSprite,
    },
    initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp},
};

#[allow(clippy::too_many_arguments)]
pub fn read_spawn_manual_select_sprite(
    // texture: &Handle<Image>,
    // layout: &Handle<TextureAtlasLayout>,
    texture_atlas_handle_building: Res<SpriteSheetManualSelectRes>,
    mut commands: Commands,
    mut send_manual_select_event: EventReader<AddTileManualSelectionSprite>,
) {
    for e in send_manual_select_event.read() {
        let ent: Entity = e.0;
        let animation_indices = AnimationIndicesComp { first: 0, last: 7 };
        let transform = Transform {
            translation: Vec3::new(TILE_SIZE.x / 2., TILE_SIZE.x / 2., 100.),
            scale: Vec3::new(3.0, 3.0, 1.0),
            ..Default::default()
        };

        commands
            .entity(ent)
            .insert(SelectedTile(true))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_handle_building.layout.clone(),
                            index: animation_indices.first,
                        }),
                        image: texture_atlas_handle_building.texture.clone(),
                        ..Default::default()
                    },
                    transform,
                    SelectionSprite,
                    animation_indices,
                    AnimationTimerComp(Timer::from_seconds(0.18, TimerMode::Repeating)),
                ));
            });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn read_despawn_manual_select_sprite(
    mut commands: Commands,
    mut entity_to_despawn_from_event: EventReader<RemoveTileManualSelectionSprite>,
    selection_q: Query<(Entity, &ChildOf), With<SelectionSprite>>,
    mut parent_q: Query<&mut SelectedTile>,
) {
    for e in entity_to_despawn_from_event.read() {
        for (ent, child_of) in selection_q.iter() {
            if e.0 == child_of.parent() {
                // Despawn the SelectionSprite child
                commands.entity(ent).despawn();

                // Override SelectedTile on parent
                if let Ok(mut selected) = parent_q.get_mut(child_of.parent()) {
                    *selected = SelectedTile(false);
                }
            }
        }
    }
}
