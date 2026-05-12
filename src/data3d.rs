use properties3d::*;
use properties3d::types3d::IJK;

use crate::gs_loader;

//  //  //  //  //  //  //  //
pub struct SelectedProperty {
    pub grid: CachedGrid,
    pub data3d: Box<[Option<f32>]>,
    pub min_value: f32,
    pub avr_value: f32,
    pub max_value: f32,
}

impl SelectedProperty {
    pub fn new(
        loaded: &gs_loader::LoadedData3D,
        property_index: usize,
        undef_value: f32,
    ) -> Self {
        let [i, j, k] = loaded.ijk;
        let grid = CachedGrid::new(i, j, k);

        let mut min_value: Option<f32> = None;
        let mut max_value: Option<f32> = None;
        let mut sum_num: Option<(f32, usize)> = None;
        let data3d = loaded.gs_data.properties[property_index]
            .clone()
            .into_iter()
            .map(|item| {
                if item <= undef_value {
                    None
                } else {
                    {
                        let v = item;
                        if min_value == None {
                            min_value = Some(v);
                        } else {
                            if v < min_value.unwrap() {
                                min_value = Some(v);
                            }
                        }
                        if max_value == None {
                            max_value = Some(v);
                        } else {
                            if v > max_value.unwrap() {
                                max_value = Some(v);
                            }
                        }
                        if let Some((sum, num)) = sum_num {
                            sum_num = Some((sum + v, num + 1));
                        } else {
                            sum_num = Some((v, 1));
                        }
                    }
                    Some(item)
                }
            })
            .collect();

        let min_value = min_value.expect("undefined min_value");
        let max_value = 1e-6 + max_value.expect("undefined max_value");
        let (sum, num) = sum_num.expect("no values to average");
        let avr_value = sum / (num as f32);

        Self {
            grid,
            data3d,
            min_value,
            avr_value,
            max_value,
        }
    }

    pub fn get_norm_value(&self, i: usize, j: usize, k: usize) -> Option<f32> {
        let ijk = IJK {
            i: i,
            j: j,
            k: k,
        };

        let Some(index) = self.grid.index_from(&ijk) else {
            return None;
        };
        let Some(v) = self.data3d[index] else {
            return None;
        };
        Some( (v-self.min_value)/(self.max_value-self.min_value) )
    }
}

