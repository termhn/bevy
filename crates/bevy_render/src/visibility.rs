use crate::render_world::{RenderWorld, ViewNode};
use bevy_core::FloatOrd;
use bevy_ecs::{Entity, Query, ResMut};
use bevy_math::Vec3;
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
pub struct RenderView {
    pub spawning_feature: Cow<'static, str>,
    pub origin: Vec3,
    pub near_plane: Plane,
    pub far_plane: Plane,
    pub left_plane: Plane,
    pub right_plane: Plane,
    pub bottom_plane: Plane,
    pub top_plane: Plane,
}

pub struct ComputedRenderView {
    pub spawning_feature: Cow<'static, str>,
    pub view_nodes: Vec<ViewNode>,
}

/// All render views to be rendered this frame. Visiblity jobs
/// will be spawned based on the views added to this resource
/// each frame
#[derive(Default)]
pub struct RenderViews {
    pub views: Vec<RenderView>,
}

/// Bounding shape centered on the entity's transform
#[derive(Debug)]
pub enum Bounds {
    Sphere { radius: f32 },
    Rect { width: f32, height: f32 },
}

/// Defines an entity as being visible within the scene, should be considered
/// in visibility checks and for rendering by rendering systems.
#[derive(Default, Debug, Properties)]
pub struct Visible {
    /// Bounding sphere centered at this entity's GlobalTransform.
    pub bounds: Bounds,
    /// Does this entity occlude other entities? i.e. is it opaque
    pub occluder: bool,
    /// Does this entity occlude other entities? i.e. is it opaque
    pub hidden: bool,
}

#[derive(Default, Debug, Properties)]
pub struct FrameVisibility {
    /// if the entity is visible to any views this frame. this value is updated automatically
    /// by the visibility system.
    #[property(ignore)]
    pub frame_visible: bool,
    // TODO: opt with smallvec or bitset?
    /// which views this entity is visible to this frame. this value is updated automatically
    /// by the visibility system.
    #[property(ignore)]
    pub visible_to_views: Vec<ViewVisibility>,
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
pub struct ViewVisibility {
    pub view_index: usize,
    pub order: FloatOrd,
}

impl ViewVisibility {
    pub fn new(view_index: usize, order: FloatOrd) -> Self {
        Self { view_index, order }
    }
}

pub fn visible_entities_system(
    mut render_views: ResMut<RenderViews>,
    mut query: Query<(Entity, &Visible, &mut FrameVisibility, &GlobalTransform)>,
) {
    // TODO: multi thread this (invert loop and go wide over views? Or could we go wide over entities?)
    for (entity, visible, mut frame_visibility, transform) in &mut query.iter() {
        frame_visibility.frame_visible = false;
        frame_visibility.visible_to_views.clear();
        for (view_index, view) in render_views.views.iter().enumerate() {
            let view_center = view.origin;

            if !visible.hidden {
                let bounds_center = transform.translation();

                let order = FloatOrd((view_center - bounds_center).length());

                // TODO: frustum culling
                frame_visibility.frame_visible = true;
                frame_visibility
                    .visible_to_views
                    .push(ViewVisibility::new(view_index, order));
            }
        }
    }
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
