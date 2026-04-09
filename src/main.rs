use kiss3d::prelude::*;
use std::f32::consts::PI;

mod command;
mod content3d;
//use content3d::*;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[kiss3d::main]
async fn main() {
    let (data3d, _start_location) = command::run();

    let mut window = Window::new(&format!("view3d: {}", data3d.name)).await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty()
        .rotate(Quat::from_axis_angle(Vec3::X, -PI / 2.))
        .rotate(Quat::from_axis_angle(Vec3::Y, PI));

    let _light_source_0 = scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 0.0, 20.0));
    let _light_source_1 = scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 40.0, -20.0));

    let font = Font::default();
    let info_text = format!(
        "max: {:1.4}\navr: {1:4}\nmin: {:1.4}",
        data3d.max_value,
        data3d.avr_value,
        data3d.min_value,
    );

    content3d::init_axes(&mut scene, 0.5);
    content3d::construct_property3d(&mut scene, 1., 0.8, &data3d);

    while window.render_3d(&mut scene, &mut camera).await {
        window.draw_text(&info_text, Vec2::ZERO, 20.0, &font, CYAN);
    }
}
