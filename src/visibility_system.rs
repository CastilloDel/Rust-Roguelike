use super::{gamelog::GameLog, Hidden, Map, Name, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player, mut hidden, mut rng, mut log, names) =
            data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| {
                    p.x >= 0 && p.x < map.width - 1 && p.y >= 0 && p.y < map.height - 1
                });
                // If this is the player, reveal what they can see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let index = map.xy_index(vis.x, vis.y);
                        map.revealed_tiles[index] = true;
                        map.visible_tiles[index] = true;

                        for e in map.tile_content[index].iter() {
                            let maybe_hidden = hidden.get(*e);
                            if let Some(_maybe_hidden) = maybe_hidden {
                                if rng.roll_dice(1, 24) == 1 {
                                    let name = names.get(*e);
                                    if let Some(name) = name {
                                        log.entries.push(format!("You spotted a {}.", &name.name));
                                    }
                                    hidden.remove(*e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
