use kiss3d::prelude::*;
use std::f32::consts::PI;

use crate::app_state;

//  //  //  //  //  //  //  //
pub struct Kiss3dState {
    window: Window,
    camera: OrbitCamera3d,
    scene: SceneNode3d,
    node_for_data: SceneNode3d,
}

impl Kiss3dState {
    pub async fn run(&mut self, mut ui_closure: impl FnMut(&egui::Context, &mut SceneNode3d) ) {
        while self
            .window
            .render_3d(&mut self.scene, &mut self.camera)
            .await
        {
            self.window.draw_ui(|ctx| {
                ui_closure(ctx, &mut self.node_for_data);
            });
        }
    }
}

//  //  //  //  //  //  //  //
pub async fn init(
    name: &str, //
    axis_size: f32,
    axis_info: &app_state::AxisInfo,
) -> Kiss3dState {
    let window = Window::new(&format!("DEBUG 4 view3d: {}", name)).await;
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

    let node_for_data = scene.add_group();

    let _node_for_axis = crate::view3d::axis3d_init(&mut scene, axis_size, axis_info);

    Kiss3dState {
        window,
        camera,
        scene,
        node_for_data,
    }
}
