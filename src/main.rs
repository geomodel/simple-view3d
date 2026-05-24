mod app_state;
use app_state::AppState;

mod config;
mod gs_loader;
mod rendering;
mod ui;
mod view3d;

//  //  //  //  //  //  //  //
mod app_consts {
    pub const SCALE: f32 = 0.8;
    pub const AXIS_SIZE: f32 = 0.5;
    pub const UNDEF: f32 = -999.0;
}

//  //  //  //  //  //  //  //
#[kiss3d::main]
async fn main() {
    let mut state = init_state();

    let mut renderer =
        rendering::init(&state.get_name(), app_consts::AXIS_SIZE, &state.axis_info).await;

    renderer
        .run(|ctx, mut data_node| {
            let mut property_index: Option<usize> = state.get_visible_index();
            let mut slice_planes = state.slice_planes.clone();
            ui::update(ctx, &state, &mut property_index, &mut slice_planes);
            if state.slice_planes != slice_planes {
                state.slice_planes = slice_planes;
                state.set_invalidate();
            }
            state.select_property(property_index);
            state.update(&mut data_node);
        })
        .await;
}

//  //  //  //  //  //  //  //
fn init_state() -> app_state::AppState {
    let config = config::parse_cli();
    let data = gs_loader::LoadedData3D::from_filename(&config.property, &config.ijk)
        .unwrap_or_else(|err| panic!("Fatal: {}", err));
    let mut state = AppState::init(data, app_consts::SCALE, config.z_scale);
    state.select_property(Some(config.property_index));

    state

}
