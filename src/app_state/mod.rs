mod structs;
use crate::gs_loader;
use crate::view3d;
pub use structs::*;

use kiss3d::prelude::SceneNode3d;

//  //  //  //  //  //  //  //
pub struct AppState {
    data: gs_loader::LoadedData3D,
    pub axis_info: AxisInfo,
    pub slice_planes: SomeIJK,
    pub property_names: Vec<String>,
    pub selected: Option<view3d::SelectedProperty>,
    visible_index: Option<usize>,
    invalidated: bool,
    property_node: Option<SceneNode3d>,
}

//  //  //  //  //  //  //  //
impl AppState {
    pub fn init(
        data: gs_loader::LoadedData3D, //
        scale: f32,
        z_scale: f32,
    ) -> AppState {
        let [i, j, k] = data.ijk;
        let axis_info = AxisInfo::new(SomeIJK { i, j, k }, scale, z_scale);

        let mut property_names = Vec::<String>::new();
        for p in data.gs_data.header.descriptions.iter() {
            let items: Vec<&str> = p.split_whitespace().collect();
            property_names.push(items[0].to_string());
        }

        AppState {
            data,
            axis_info,
            slice_planes: SomeIJK { i: 0, j: j-1, k: 0 },
            property_names,
            selected: None,
            visible_index: None,
            invalidated: false,
            property_node: None,
        }
    }
}

//  //  //  //  //  //  //  //
impl AppState {
    pub fn get_name(&self) -> &str {
        &self.data.name
    }
    pub fn get_info_text(&self) -> String {
        match &self.selected {
            Some(data3d) => {
                format!(
                    "min: {:1.4}\navr: {:1.4}\nmax: {:1.4}",
                    data3d.min_value, data3d.avr_value, data3d.max_value,
                )
            }
            None => {
                format!("min: -\navr: -\nmax: -",)
            }
        }
    }

    pub fn set_invalidate(&mut self) {
        self.invalidated = true;
    }

    pub fn get_visible_index(&self) -> Option<usize> {
        self.visible_index
    }
    pub fn select_property(&mut self, select_index: Option<usize>) {
        if self.visible_index != select_index {
            //self.invalidated = true;
            self.set_invalidate();
            self.visible_index = select_index;
        }
    }

    fn clear_node(&mut self) {
        if let &mut Some(ref mut v) = &mut self.property_node {
            v.remove();
            self.property_node = None;
        }
    }

    pub fn update(&mut self, scene: &mut SceneNode3d) {
        if self.invalidated {
            self.clear_node();
            //
            let selected = if let Some(index) = self.visible_index {
                let some_selected =
                    view3d::SelectedProperty::new(&self.data, index, crate::app_consts::UNDEF);
                //
                {
                    let v = view3d::construct_property3d(&self.axis_info, &self.slice_planes, scene, &some_selected);
                    self.property_node = Some(v);
                }
                //
                Some(some_selected)
            } else {
                None
            };
            self.selected = selected;
        }
        self.invalidated = false;
    }
}
