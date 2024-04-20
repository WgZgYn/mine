use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Reflect, InspectorOptions, Copy, Clone, Eq, PartialEq, Debug, Default)]
#[reflect(Component, InspectorOptions)]
pub enum CellType {
    #[default]
    Empty,
    Mine,
    Number(usize),
}

#[derive(Component, Copy, Clone, Debug, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub enum CellState {
    #[default]
    Covered,
    Uncovered,
    Flaged,
}
