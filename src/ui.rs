use crate::view3d::SelectedProperty;
use crate::app_state::AppState;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub(crate) fn update(
    ctx: &egui::Context, //
    state: &mut AppState,
    //data3d: Option<&SelectedProperty>,
    //trigger: &mut bool,
) {
    egui::Window::new("inspector")
        .default_pos([0.0, 0.0])
        .default_width(300.0)
        .show(ctx, |ui| {
            //label_info(ui, state.selected);
            /*
            if ui.button("testMe").clicked() {
                *trigger = true;
            }
            */
            ui.separator();
            ui.horizontal(|ui| {
                for (i, s) in state.property_names.clone().iter().enumerate() {
                    if ui.button(s).clicked() {
                        state.select_property(Some(i));
                    }
                }
            });
        });
}

//  //  //  //  //  //  //  //
fn label_info(ui: &mut egui::Ui, data3d: Option<&SelectedProperty>) {
    let info_text = match data3d {
        Some(data3d) => {
            format!(
                "min: {:1.4}\navr: {:1.4}\nmax: {:1.4}",
                data3d.min_value, data3d.avr_value, data3d.max_value,
            )
        },
        None => {
            format!(
                "min: -\navr: -\nmax: -",
            )
        },
    };
    ui.label(info_text);
}
