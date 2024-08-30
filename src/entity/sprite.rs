use bevy::prelude::{Resource, SpriteSheetBundle};

#[derive(Resource, Default)]
pub struct SpriteContainer {
    sprite: Vec<SpriteSheetBundle>,
}

impl SpriteContainer {
    pub fn push(&mut self, sp: SpriteSheetBundle) {
        self.sprite.push(sp);
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut SpriteSheetBundle> {
        self.sprite.get_mut(i)
    }
}
