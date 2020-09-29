use crate::visibility::{RenderView, ViewVisibleEntity};
use bevy_ecs::{Entity, ParallelExecutor, Resources, Schedule, World};

#[derive(Debug, Clone)]
pub struct RenderWorldEntity(pub Entity);
#[derive(Default)]
pub struct RenderWorld {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
    pub executor: ParallelExecutor,
}
