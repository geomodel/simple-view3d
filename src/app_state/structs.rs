//  //  //  //  //  //  //  //
#[derive(Clone, PartialEq)]
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
