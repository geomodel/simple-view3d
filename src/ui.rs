use crate::app_state::{AppState, SomeIJK};

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub(crate) fn update(
    ctx: &egui::Context, //
    state: &AppState,
    out_property_index: &mut Option<usize>,
    out_slice_planes: &mut SomeIJK,
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

            ui.separator();
            let ijk = &state.axis_info.ijk;
            ui.add(egui::Slider::new(&mut out_slice_planes.i, 0..=(ijk.i-1)).text("I"));
            ui.add(egui::Slider::new(&mut out_slice_planes.j, 0..=(ijk.j-1)).text("J"));
            ui.add(egui::Slider::new(&mut out_slice_planes.k, 0..=(ijk.k-1)).text("K"));
        });
}
