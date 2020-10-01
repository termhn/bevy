use crate::{
    camera::{Camera, OrthographicProjection, PerspectiveProjection},
    pipeline::RenderPipelines,
    render_graph::base,
    Draw, FrameVisibility, Mesh, Visible,
};
use base::MainPass;
use bevy_asset::Handle;
use bevy_ecs::{Bundle, Entity, EntityBuilder};
use bevy_math::Vec3;
use bevy_property::Properties;
use bevy_transform::components::{GlobalTransform, Transform};

/// A builder which can be used to extract data from the simulation World
/// into the RenderWorld.
#[derive(Default, Properties)]
pub struct RenderEntityBuilder {
    #[property(ignore)]
    pub builder: EntityBuilder,
}

/// If it exists, the corresponding Entity in the RenderWorld
#[derive(Default, Properties)]
pub struct RenderWorldEntity {
    #[property(ignore)]
    pub render_entity: Option<Entity>,
}

/// A component bundle for "mesh" entities
#[derive(Bundle, Default)]
pub struct MeshComponents {
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visible: Visible,
    pub visibility: FrameVisibility,
    pub render_entity_builder: RenderEntityBuilder,
    pub render_world_entity: RenderWorldEntity,
}

/// A component bundle for "3d camera" entities
#[derive(Bundle)]
pub struct Camera3dComponents {
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for Camera3dComponents {
    fn default() -> Self {
        Camera3dComponents {
            camera: Camera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// A component bundle for "2d camera" entities
#[derive(Bundle)]
pub struct Camera2dComponents {
    pub camera: Camera,
    pub orthographic_projection: OrthographicProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for Camera2dComponents {
    fn default() -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        Camera2dComponents {
            camera: Camera {
                name: Some(base::camera::CAMERA2D.to_string()),
                ..Default::default()
            },
            orthographic_projection: OrthographicProjection {
                far,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, far - 0.1)),
            global_transform: Default::default(),
        }
    }
}
