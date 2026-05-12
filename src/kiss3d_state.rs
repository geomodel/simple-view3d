use kiss3d::prelude::*;
use std::f32::consts::PI;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct Kiss3dState {
    pub window: Window,
    pub camera: OrbitCamera3d,
    pub scene: SceneNode3d,
}

impl Kiss3dState {
    pub async fn new(name: &str) -> Self {
        let window = Window::new(&format!("DEBUG 3 view3d: {}", name)).await;
        let camera = OrbitCamera3d::default();
        let mut scene = SceneNode3d::empty()
            .rotate(Quat::from_axis_angle(Vec3::X, -PI / 2.))
            .rotate(Quat::from_axis_angle(Vec3::Y, PI));

        let _light_source_0 = scene
            .add_light(Light::point(100.0))
            .set_position(Vec3::new(0.0, 0.0, 20.0));
        let _light_source_1 = scene
            .add_light(Light::point(100.0))
            .set_position(Vec3::new(0.0, 40.0, -20.0));

        Kiss3dState {
            window,
            camera,
            scene,
        }
    }
}
