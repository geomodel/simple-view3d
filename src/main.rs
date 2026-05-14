mod config;
mod gs_loader;
mod rendering;
mod app_state;
mod ui;

mod view3d;

const SCALE: f32 = 0.8;
const AXIS_SIZE: f32 = 0.5;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[kiss3d::main]
async fn main() {
    let config = config::parse_cli();
    let data = gs_loader::LoadedData3D
        ::from_filename(&config.property, &config.ijk)
            .unwrap_or_else(|err| panic!("Fatal: {}", err));
    let mut state = app_state::init(SCALE, config.z_scale, &data.ijk);

    let mut renderer = rendering::init(&data.name, AXIS_SIZE, &state.axis_info).await;

    let selected: usize = 0;
    let mut trigger = false;
    renderer.run( |ctx| {
        ui::update(ctx, &selected, &mut trigger);
    }).await;

            //todo!("run-loop")
            /*
            // egUI
            let mut trigger = false;
            kiss3d_state.window.draw_ui(|ctx| {
                ui::update(ctx, &selected, &mut trigger);
            });
            if trigger {
                state.trigger(&selected, &mut kiss3d_state.scene);
            }
            */
    //state.reshow_axis3d(&mut kiss3d_state.scene);
}
