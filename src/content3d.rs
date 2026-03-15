use std::f32::consts::PI;
use kiss3d::prelude::*;

use properties3d::*;


//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub fn init_axes(scene: &mut SceneNode3d, size: f32) {
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
}

pub fn construct_property3d(
    scene: &mut SceneNode3d,
    z_scale: f32,
    scale: f32,
    data3d: &crate::command::Data3D,
){
    let i_max = data3d.grid.i_max() as f32;
    let j_max = data3d.grid.j_max() as f32;
    let k_max = data3d.grid.k_max() as f32;

    let i_size = i_max;
    let j_size = j_max;
    let k_size = z_scale*k_max;

    let max_size = if i_size > j_size { i_size } else { j_size };
    let max_size = if k_size > max_size { k_size } else { max_size };
    let max_size = max_size;

    let i_size = scale * i_size / max_size;
    let j_size = scale * j_size / max_size;
    let k_size = scale * k_size / max_size;

    let di = i_size / i_max as f32;
    let dj = j_size / j_max as f32;
    let dk = k_size / k_max as f32;

    let start_i = (di - i_size) / 2.;
    let start_j = (dj - j_size) / 2.;
    let start_k = (dk - k_size) / 2.;

    let j_max_1 = data3d.grid.i_max() - 1;
    let i_mid = data3d.grid.i_max() / 3;
    let j_mid = data3d.grid.j_max() * 2 / 3;
    let k_mid = data3d.grid.k_max() / 3;

    for i in 0..data3d.grid.i_max() {
        for j in 0..data3d.grid.j_max() {
            for k in 0..data3d.grid.k_max() {
                if (i == 0) | (j == j_max_1) | (k == 0) 
                    | (i == i_mid) | (j == j_mid) | (k == k_mid) {
                //{
                    if let Some(v) = data3d.get_norm_value(i,j,k) {
                        let c0 = Color::new( 0.4, 0.0, 1.0, 1.0 );
                        let c1 = Color::new( 0.1, 0.1, 1.0, 1.0 );
                        let c2 = Color::new( 0.0, 1.0, 0.0, 1.0 );
                        let c3 = Color::new( 1.0, 1.0, 0.0, 1.0 );
                        let c4 = Color::new( 1.0, 0.0, 0.0, 1.0 );
                        let color = match v {
                            0.0..0.25 => {
                                mix_colors( v*4., &c0, &c1 )
                            },
                            0.25..0.50 => {
                                mix_colors( (v-0.25)*4., &c1, &c2 )
                            },
                            0.50..0.75 => {
                                mix_colors( (v-0.50)*4., &c2, &c3 )
                            },
                            _ => {
                                mix_colors( (v-0.75)*4., &c3, &c4 )
                            },
                        };
                        let f_i = i as f32;
                        let f_j = j as f32;
                        let f_k = k as f32;
                        scene
                            .add_cube(di,dj,dk)
                            //.add_sphere(1.01).set_local_scale(di,dj,dk)
                            .set_position(Vec3::new(
                                        start_i + di * f_i,
                                        start_j + dj * f_j,
                                        start_k + dk * f_k,
                                    ))
                            .set_color(color);
                    }
                }
            }
        }
    }
    scene.add_cube(i_size*1.001, j_size*1.001, k_size*1.001)
            .set_color( Color::new_alpha( 0.5, 0.5, 0.5, 0.33) );
}

fn mix_colors( w: f32, min_color: &Color, max_color: &Color) -> Color {
    let uw = 1.-w*w;
    let uw = 1.-uw;
    let uw = 1.-uw*uw;
    Color::new(
        uw*min_color.r + w*max_color.r,
        uw*min_color.g + w*max_color.g,
        uw*min_color.b + w*max_color.b,
        uw*min_color.a + w*max_color.a,
    )
}

