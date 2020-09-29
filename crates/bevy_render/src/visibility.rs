use bevy_math::Vec3;
use bevy_property::Properties;

use std::borrow::Cow;

// A plane as defined by the equation
// `ax + by + cz + d = 0`
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
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

/// All render views to be rendered this frame. Visiblity jobs
/// will be spawned based on the views added to this resource
/// each frame.
#[derive(Default)]
pub struct RenderViews {
    pub views: Vec<RenderView>,
}

#[derive(Default, Debug, Properties)]
pub struct BoundingSphere {
    pub radius: f32,
}
/// Defines an entity as being visible within the scene, should be considered
/// in visibility checks and for rendering by rendering systems.
#[derive(Default, Debug, Properties)]
pub struct Visible {
    // TODO: Add bounding boxes?
    /// Bounding sphere centered at this entity's GlobalTransform.
    pub bounds: BoundingSphere,
    /// Does this entity occlude other entities? i.e. is it opaque
    pub occluder: bool,
    /// If set to true, will automatically be discarded from visibility testing.
    #[property(ignore)]
    pub hidden: bool,
}

// /// Defines an entity as dynamic, i.e. able to move.
// pub struct Dynamic {}

// /// Defines an entity as static, i.e. unable to move. The renderer may be able to
// /// perform significant optimizations with the assumption that most of a scene
// /// is static.
// pub struct Static {}
