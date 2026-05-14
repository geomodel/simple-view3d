//use crate::data3d::SelectedProperty;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//pub(crate) fn update(ctx: &egui::Context, data3d: &SelectedProperty, trigger: &mut bool) {
pub(crate) fn update(ctx: &egui::Context, data3d: &usize, trigger: &mut bool) {
    egui::Window::new("inspector")
        .default_pos([0.0, 0.0])
        .default_width(300.0)
        .show(ctx, |ui| {
            label_info(ui, data3d);
            if ui.button("testMe").clicked() {
                *trigger = true;
            }
        });
}

//  //  //  //  //  //  //  //
//fn label_info(ui: &mut egui::Ui, data3d: &SelectedProperty) {
fn label_info(ui: &mut egui::Ui, _data3d: &usize) {
    /*
    let info_text = format!(
        "min: {:1.4}\navr: {:1.4}\nmax: {:1.4}",
        data3d.min_value, data3d.avr_value, data3d.max_value,
    );
    */
    let info_text = "<none>";
    ui.label(info_text);
}
