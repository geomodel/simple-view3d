use kiss3d::prelude::*;

use crate::data3d;
use crate::content3d;
use crate::axis3d;

//  //  //  //  //  //  //  //
pub struct AppState {
    pub axis_info: AxisInfo,
    pub prop_group: Option<kiss3d::prelude::SceneNode3d>,
    pub axis3d: kiss3d::prelude::SceneNode3d,
}
impl AppState {
    pub fn new(scale: f32, z_scale: f32, ijk: &[usize; 3]) -> Self {
        let [i, j, k] = *ijk;
        let axis_info = AxisInfo::new(
            SomeIJK {
                i,
                j,
                k,
            },
            scale,
            z_scale,
        );

        Self {
            axis_info,
            prop_group: None,
            axis3d: SceneNode3d::empty(),
        }
    }

    pub fn trigger(&mut self, selected: &data3d::SelectedProperty, scene: &mut SceneNode3d) {
        println!("triggered");
        if let &mut Some(ref mut v) = &mut self.prop_group {
            v.remove();
            self.prop_group = None;
            println!("--> None");
        } else {
            let v = content3d::construct_property3d(&self.axis_info, scene, selected);
            self.prop_group = Some(v);
            println!("--> Some");
        }
        self.reshow_axis3d(scene);
    }

    pub fn reshow_axis3d(&mut self, scene: &mut SceneNode3d) {
        self.axis3d.remove();
        self.axis3d = axis3d::init(scene, 0.5, &self.axis_info);
    }
}

//  //  //  //  //  //  //  //
pub struct SomeIJK {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}
pub struct SomeFloatIJK {
    pub fi: f32,
    pub fj: f32,
    pub fk: f32,
}

pub struct AxisInfo {
    pub ijk: SomeIJK,
    pub size: SomeFloatIJK,
    pub d: SomeFloatIJK,
    pub start: SomeFloatIJK,
}

impl AxisInfo {
    pub fn new(ijk: SomeIJK, scale: f32, z_scale: f32) -> Self {
        let fi = ijk.i as f32;
        let fj = ijk.j as f32;
        let fk = ijk.k as f32;
        let size = SomeFloatIJK {
            fi: fi,
            fj: fj,
            fk: fk * z_scale,
        };
        let max_length = size.fi.max(size.fj).max(size.fk);

        let size = SomeFloatIJK {
            fi: scale * size.fi / max_length,
            fj: scale * size.fj / max_length,
            fk: scale * size.fk / max_length,
        };

        let d = SomeFloatIJK {
            fi: size.fi / fi,
            fj: size.fj / fj,
            fk: size.fk / fk,
         };
        let start = SomeFloatIJK {
            fi: (d.fi - size.fi) / 2.0,
            fj: (d.fj - size.fj) / 2.0,
            fk: (d.fk - size.fk) / 2.0,
        };

        AxisInfo {
            ijk,
            size,
            d,
            start,
        }
    }
}
