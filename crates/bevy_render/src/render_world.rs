use crate::visibility::{ComputedRenderView, RenderView};
use bevy_core::FloatOrd;
use bevy_ecs::{prelude::*, Entity, EntityBuilder, ParallelExecutor, Resources, Schedule, World};
use bevy_property::Properties;

use core::any::TypeId;
use std::collections::{hash_map::Entry, HashMap};

use downcast_rs::Downcast;
pub use froggy::{Pointer as RenderDataPointer, Storage as RenderFeatureStorage};

/// Types that can be used as data in the RenderWorld. Automatically implemented
/// for all types that are `Send + Sync + 'static`.
pub trait RenderData: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> RenderData for T {}

// /// A type-erased version of a `RenderDataPointer` which maybe downcasted into
// /// a typed version.
// pub trait DynRenderDataPointer: Downcast {}
// downcast_rs::impl_downcast!(DynRenderDataPointer);
// impl<T: RenderData> DynRenderDataPointer for RenderDataPointer<T> {}

/// A thin representation of a visible entity in a single view which
/// points back to the RenderWorld Entity with pointers to its associated data
/// and contains ordering information relative to that view.
pub struct ViewNode {
    pub render_entity: Entity,
    pub order: FloatOrd,
}

#[derive(Default)]
pub struct RenderWorld {
    pub world: World,
    pub resources: Resources,
}

impl RenderWorld {
    pub fn new() -> Self {
        let mut rw = Self {
            world: World::new(),
            resources: Resources::default(),
        };

        rw.resources.insert(Vec::<RenderView>::new());
        rw.resources.insert(Vec::<ComputedRenderView>::new());

        rw
    }

    // pub fn get_storage<T: RenderData>(&self) -> Option<Ref<'_, RenderFeatureStorage<T>>> {
    //     self.resources.get()
    // }

    // pub fn get_storage_mut<T: RenderData>(&self) -> Option<RefMut<'_, RenderFeatureStorage<T>>> {
    //     self.resources.get_mut()
    // }
}
