use bevy_ecs::{Entity, EntityBuilder, ParallelExecutor, Resources, Schedule, World};

use core::any::TypeId;
use std::collections::{hash_map::Entry, HashMap};

use downcast_rs::Downcast;
use generational_arena::{Arena, Index};

pub struct RenderEntityBuilder {
    builder: EntityBuilder,
}

#[derive(Default)]
pub struct RenderWorld {
    world: World,
}

// pub trait RenderData: Send + Sync + 'static {}
// impl<T: Send + Sync + 'static> RenderData for T {}

// pub struct Storage<T> {
//     arena: Arena<T>,
// }

// impl<T> Storage<T> {
//     pub fn new() -> Self {
//         Self {
//             arena: Arena::new(),
//         }
//     }

//     pub fn get(&self, entity: Index) -> Option<&T> {
//         self.arena.get(entity.0)
//     }
// }
// pub trait RenderDataStorage: RenderData + Downcast {}
// downcast_rs::impl_downcast!(RenderDataStorage);
// impl<T: RenderData + 'static> RenderDataStorage for Storage<T> {}

// pub enum RenderWorldError {
//     StorageDoesNotExist,
// }

// pub struct RenderWorld {
//     dynamic_render_data: HashMap<TypeId, Box<dyn RenderDataStorage>>,
// }

// impl RenderWorld {
//     pub fn register<T: RenderData>(&mut self) {
//         self.dynamic_render_data
//             .insert(TypeId::of::<T>(), Box::new(Storage::<T>::new()));
//     }

//     pub fn get_storage<T: RenderData>(&self) -> Result<&Storage<T>, RenderWorldError> {
//         Ok(self
//             .dynamic_render_data
//             .get(&TypeId::of::<T>())
//             .ok_or(RenderWorldError::StorageDoesNotExist)?
//             .downcast_ref()
//             .unwrap())
//     }

//     pub fn get_storage_mut<T: RenderData>(&self) -> Result<&mut Storage<T>, RenderWorldError> {
//         Ok(self
//             .dynamic_render_data
//             .get(&TypeId::of::<T>())
//             .ok_or(RenderWorldError::StorageDoesNotExist)?
//             .downcast_ref()
//             .unwrap())
//     }
// }
