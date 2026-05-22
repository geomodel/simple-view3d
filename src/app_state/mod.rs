mod structs;
pub use structs::*;
use crate::gs_loader;
use crate::view3d;

//  //  //  //  //  //  //  //
pub fn init(
    data: gs_loader::LoadedData3D, //
    scale: f32,
    z_scale: f32,
) -> AppState {
    let [i, j, k] = data.ijk;
    let axis_info = AxisInfo::new(SomeIJK { i, j, k }, scale, z_scale);

    AppState {
        data,
        axis_info,
        property_names: Vec::new(),
        selected: None, //view3d::SelectedProperty::new(&data, 0, UNDEF),
        visible_index: None,
        invalidated: false,
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct AppState {
    data: gs_loader::LoadedData3D,
    pub axis_info: AxisInfo,
    pub property_names: Vec<String>,
    pub selected: Option<view3d::SelectedProperty>,
    visible_index: Option<usize>,
    invalidated: bool,
}

impl AppState {
    pub fn get_name(&self) -> &str {
        &self.data.name
    }

    pub fn select_property(&mut self, select_index: Option<usize>) {
        if self.visible_index != select_index {
            self.invalidated = true;
            self.visible_index = select_index;
            println!("select request :{:?}", select_index);
        }
    }

    /*
    pub fn check_trigger(&mut self, scene: &mut SceneNode3d) {
        if self.invalidated {
            {
                println!("triggered");
                if let &mut Some(ref mut v) = &mut self.property_node {
                    //v.remove();
                    //self.property_node = None;
                    println!("--> None");
                } else {
                    //let v = view3d::construct_property3d(&self.axis_info, scene, self.selected);
                    //self.property_node = Some(v);
                    println!("--> Some");
                }
            }
            self.invalidated = false;
        }
    }
    */
}
