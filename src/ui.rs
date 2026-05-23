use crate::app_state::AppState;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub(crate) fn update(
    ctx: &egui::Context, //
    state: &AppState,
    out_property_index: &mut Option<usize>,
) {
    egui::Window::new("inspector")
        .default_pos([0.0, 0.0])
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.label(state.get_info_text());

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("RESET").clicked() {
                    *out_property_index = None;
                }
                for (i, s) in state.property_names.clone().iter().enumerate() {
                    if ui.button(s).clicked() {
                        *out_property_index = Some(i);
                    }
                }
            });
        });
}
