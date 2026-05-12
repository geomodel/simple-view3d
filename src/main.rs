mod cli;
mod data3d;
mod gs_loader;
mod ui;
mod kiss3d_state;
mod axis3d;
mod app_state;
mod content3d;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[kiss3d::main]
async fn main() {
    let (gs_loaded, selected, z_scale) = cli::parse_n_load();

    let mut kiss3d_state = kiss3d_state::Kiss3dState::new(&gs_loaded.name).await;

    let mut state = app_state::AppState::new(0.8, z_scale, &gs_loaded.ijk);
    state.reshow_axis3d(&mut kiss3d_state.scene);

    while kiss3d_state.window.render_3d(&mut kiss3d_state.scene, &mut kiss3d_state.camera).await {
        // egUI
        let mut trigger = false;
        kiss3d_state.window.draw_ui(|ctx| {
            ui::update(ctx, &selected, &mut trigger);
        });
        if trigger {
            state.trigger(&selected, &mut kiss3d_state.scene);
        }
    }
}
