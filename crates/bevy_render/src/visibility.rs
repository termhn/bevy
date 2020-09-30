use bevy_math::Vec3;
use crate::{RenderWorld, RenderWorldEntity};
use bevy_core::FloatOrd;
use bevy_ecs::{Entity, Query, ResMut};
use bevy_property::Properties;
use bevy_transform::prelude::GlobalTransform;


use std::borrow::Cow;

/// A plane as defined by a normal vector and point on the plane
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub norm: Vec3,
    pub point: Vec3,
}

/// A view that needs to be rendered. Defines the needed
/// information to compute visibility information for entities
/// (frustum and visibility culling).
#[derive(Clone, Debug)]
pub struct RenderView3d {
    pub spawning_feature: Cow<'static, str>,
    pub origin: Vec3,
    pub near_plane: Plane,
    pub far_plane: Plane,
    pub left_plane: Plane,
    pub right_plane: Plane,
    pub bottom_plane: Plane,
    pub top_plane: Plane,
}

/// A view that needs to be rendered. Defines the needed
/// information to compute visibility information for entities
/// (frustum culling).
#[derive(Clone, Debug)]
pub struct RenderView2d {
    pub spawning_feature: Cow<'static, str>,
    pub origin: Vec3,
    pub right_edge: f32,
    pub left_edge: f32,
    pub bottom_edge: f32,
    pub top_edge: f32,
}

/// All 3d render views to be rendered this frame. Visiblity jobs
/// will be spawned based on the views added to this resource
/// each frame.
#[derive(Default)]
pub struct RenderViews3d {
    pub views: Vec<RenderView3d>,
}

/// All 2d render views to be rendered this frame. Visiblity jobs
/// will be spawned based on the views added to this resource
/// each frame.
#[derive(Default)]
pub struct RenderViews2d {
    pub views: Vec<RenderView2d>,
}

/// Bounding sphere centered on the entity's transform
#[derive(Default, Debug, Properties)]
pub struct BoundingSphere {
    pub radius: f32,
}

/// Bounding rect centered on the entity's transform
#[derive(Default, Debug, Properties)]
pub struct BoundingRect {
    pub width: f32,
    pub height: f32,
}

/// Defines an entity as being visible within the scene, should be considered
/// in visibility checks and for rendering by rendering systems.
#[derive(Default, Debug, Properties)]
pub struct Visible3d {
    /// Bounding sphere centered at this entity's GlobalTransform.
    pub bounds: BoundingSphere,
    /// Does this entity occlude other entities? i.e. is it opaque
    #[property(ignore)]
    pub hidden: bool,
    /// if the entity is visible to any views this frame. this value is updated automatically
    /// by the visibility system.
    pub frame_visible: bool,
    // TODO: opt with smallvec or bitset?
    /// which views this entity is visible to this frame. this value is updated automatically
    /// by the visibility system.
    pub visible_to_views: Vec<usize>,
}

/// Defines an entity as being visible within the scene, should be considered
/// in visibility checks and for rendering by rendering systems.
#[derive(Default, Debug, Properties)]
pub struct Visible2d {
    /// Bounding sphere centered at this entity's GlobalTransform.
    pub bounds: BoundingRect,
    /// Does this entity occlude other entities? i.e. is it opaque
    pub occluder: bool,
    /// If set to true, will automatically be discarded from visibility testing.
    #[property(ignore)]
    pub hidden: bool,
    /// if the entity is visible to any views this frame. this value is updated automatically
    /// by the visibility system.
    pub frame_visible: bool,
    /// which views this entity is visible to this frame. this value is updated automatically
    /// by the visibility system.
    // TODO: opt with smallvec or bitset?
    pub visible_to_views: Vec<usize>,
}

/// Defines an entity as dynamic, i.e. able to move.
#[derive(Default, Debug, Properties)]
pub struct Dynamic {}

/// Defines an entity as static, i.e. unable to move. The renderer may be able to
/// perform significant optimizations with the assumption that most of a scene
/// is static.
#[derive(Default, Debug, Properties)]
pub struct Static {}

#[derive(Debug)]
pub struct ViewVisibleEntity {
    pub entity: RenderWorldEntity,
    // distance from associated view center to 
    pub order: FloatOrd,
}

/*
pub fn visible_entities_system_old(
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut VisibleEntities)>,
    mut draw_query: Query<(Entity, &Draw)>,
    draw_transform_query: Query<(&Draw, &GlobalTransform)>,
) {
    for (_camera, camera_global_transform, mut visible_entities) in &mut camera_query.iter() {
        visible_entities.value.clear();
        let camera_position = camera_global_transform.translation();

        let mut no_transform_order = 0.0;
        let mut transparent_entities = Vec::new();
        for (entity, draw) in &mut draw_query.iter() {
            if !draw.is_visible {
                continue;
            }

            let order =
                if let Ok(global_transform) = draw_transform_query.get::<GlobalTransform>(entity) {
                    let position = global_transform.translation();
                    // smaller distances are sorted to lower indices by using the distance from the camera
                    FloatOrd((camera_position - position).length())
                } else {
                    let order = FloatOrd(no_transform_order);
                    no_transform_order += 0.1;
                    order
                };

            if draw.is_transparent {
                transparent_entities.push(VisibleEntity { entity, order })
            } else {
                visible_entities.value.push(VisibleEntity { entity, order })
            }
        }

        // sort opaque entities front-to-back
        visible_entities.value.sort_by_key(|e| e.order);

        // sort transparent entities back-to-front
        transparent_entities.sort_by_key(|e| -e.order);
        visible_entities.value.extend(transparent_entities);

        // TODO: check for big changes in visible entities len() vs capacity() (ex: 2x) and resize to prevent holding unneeded memory
    }
}
*/

pub fn visible_entities_system(
    mut render_views: ResMut<RenderViews>,
    mut render_world: ResMut<RenderWorld>,
    mut query: Query<(Entity, &mut Visible, &GlobalTransform)>,
) {
    // TODO: figure out what to do about entities with no transform that still need to be drawn? ask cart which entities these would be
    // TODO: multi thread this (invert loop and go wide over views? Or could we go wide over entities?)
    for (entity, mut visible, transform) in &mut query.iter() {
        for view in render_views.views.iter() {
            let view_center = view.origin;

            if !visible.hidden {
                let bounds_center = transform.translation();

                let obj_to_right_plane = 
                // TODO: frustum culling
                let order = FloatOrd((view_center - transform.translation()).length());
            }

            if draw.is_transparent {
                transparent_entities.push(VisibleEntity { entity, order })
            } else {
                visible_entities.value.push(VisibleEntity { entity, order })
            }
        }

        // sort opaque entities front-to-back
        visible_entities.value.sort_by_key(|e| e.order);

        // sort transparent entities back-to-front
        transparent_entities.sort_by_key(|e| -e.order);
        visible_entities.value.extend(transparent_entities);

        // TODO: check for big changes in visible entities len() vs capacity() (ex: 2x) and resize to prevent holding unneeded memory
    }
}