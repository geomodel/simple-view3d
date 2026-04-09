use properties3d::*;
use properties3d::types3d::*;

//  //  //  //  //  //  //  //
pub struct Data3D {
    pub name: String,
    pub grid: CachedGrid,
    pub property: Property<Continuous>,
    pub min_value: f32,
    pub avr_value: f32,
    pub max_value: f32,
}

impl Data3D {
    pub fn get_norm_value(&self, i: usize, j: usize, k: usize) -> Option<f32> {
        let ijk = IJK {
            i: i,
            j: j,
            k: k,
        };

        let Some(index) = self.grid.index_from(&ijk) else {
            return None;
        };
        let Some(v) = self.property[index] else {
            return None;
        };
        Some( ((v as f32)-self.min_value)/(self.max_value-self.min_value) )
    }
}
