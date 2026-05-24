use kiss3d::prelude::*;

use crate::app_state;

mod coloring;

//  //  //  //  //  //  //  //
pub fn construct_property3d(
    axis: &app_state::AxisInfo,
    slice_plaines: &app_state::SomeIJK,
    parent_scene: &mut SceneNode3d,
    data3d: &super::SelectedProperty,
) -> SceneNode3d {
    let mut scene = parent_scene.add_group();
    for i in 0..axis.ijk.i {
        for j in 0..axis.ijk.j {
            for k in 0..axis.ijk.k {
                if (i == slice_plaines.i) | (j == slice_plaines.j) | (k == slice_plaines.k) {
                    //{
                    if let Some(v) = data3d.get_norm_value(i, j, k) {
                        let color = coloring::calculate_color(v);
                        let f_i = i as f32;
                        let f_j = j as f32;
                        let f_k = k as f32;
                        scene
                            .add_cube(axis.d.fi, axis.d.fj, axis.d.fk)
                            //.add_sphere(1.01).set_local_scale(di,dj,dk)
                            .set_position(Vec3::new(
                                axis.start.fi + axis.d.fi * f_i,
                                axis.start.fj + axis.d.fj * f_j,
                                axis.start.fk + axis.d.fk * f_k,
                            ))
                            .set_color(color);
                    }
                }
            }
        }
    }
    scene
}

//  //  //  //  //  //  //  //
/*
pub fn construct_property3d_old(
    axis: &app_state::AxisInfo,
    parent_scene: &mut SceneNode3d,
    data3d: &super::SelectedProperty,
) -> SceneNode3d {

    let j_max_1 = axis.ijk.j - 1;
    let i_mid = axis.ijk.i / 2 - 1;
    let j_mid = axis.ijk.j / 2 - 1;
    let k_mid = axis.ijk.k / 2 - 1;

    let mut scene = parent_scene.add_group();
    for i in 0..axis.ijk.i {
        for j in 0..axis.ijk.j {
            for k in 0..axis.ijk.k {
                if (i == 0) | (j == j_max_1) | (k == 0)
                    | (i == i_mid) | (j == j_mid) | (k == k_mid) {
                //{
                    if let Some(v) = data3d.get_norm_value(i,j,k) {
                        let color = coloring::calculate_color(v);
                        let f_i = i as f32;
                        let f_j = j as f32;
                        let f_k = k as f32;
                        scene
                            .add_cube(axis.d.fi,axis.d.fj,axis.d.fk)
                            //.add_sphere(1.01).set_local_scale(di,dj,dk)
                            .set_position(Vec3::new(
                                        axis.start.fi + axis.d.fi*f_i,
                                        axis.start.fj + axis.d.fj*f_j,
                                        axis.start.fk + axis.d.fk*f_k,
                                    ))
                            .set_color(color);
                    }
                }
            }
        }
    }
    scene
}
*/
