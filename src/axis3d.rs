use kiss3d::prelude::*;
use std::f32::consts::PI;

use crate::app_state;

//  //  //  //  //  //  //  //
pub fn init(
    parent_scene: &mut SceneNode3d,
    size: f32,
    axis: &app_state::AxisInfo,
) -> SceneNode3d {
    let mut scene = parent_scene.add_group();

    let _origin = scene.add_sphere(size*0.015).set_color(WHITE);

    let _ii   = scene.add_cylinder(size*0.005, size*2.)
                    .rotate( Quat::from_axis_angle(Vec3::Z, -PI/2.) )
                    .set_color(GREEN);
    let _idir = scene.add_cone(size*0.02, size*0.1)
                    .rotate( Quat::from_axis_angle(Vec3::Z, -PI/2.) )
                    .set_position(Vec3::new(size*1.0, 0.0, 0.0))
                    .set_color(GREEN);

    let _jj   = scene.add_cylinder(size*0.005, size*2.)
                    .set_color(RED);
    let _jdir = scene.add_cone(size*0.02, size*0.1)
                    .set_position(Vec3::new(0.0, size*1.0, 0.0))
                    .set_color(RED);

    let _kk   = scene.add_cylinder(size*0.005, size*2.)
                    .rotate( Quat::from_axis_angle(Vec3::X, PI/2.) )
                    .set_color(BLUE);
    let _kdir = scene.add_cone(size*0.02, size*0.1)
                    .rotate( Quat::from_axis_angle(Vec3::X, PI/2.) )
                    .set_position(Vec3::new(0.0, 0.0, size*1.0))
                    .set_color(BLUE);

    scene.add_cube(axis.size.fi*1.001, axis.size.fj*1.001, axis.size.fk*1.001)
            .set_color( Color::new_alpha( 0.5, 0.5, 0.5, 0.33) );

    scene
}
