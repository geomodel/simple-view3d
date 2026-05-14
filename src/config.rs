use clap::Parser;

//use crate::data3d;
//use crate::gs_loader;

//const UNDEF: f32 = -999.0;

//  //  //  //  //  //  //  //
#[derive(clap::Parser, Debug)]
#[command(about)]
pub struct CliArgs {
    #[arg(long, default_value_t = 0)]
    pub property_index: usize,
    #[arg(short, long, default_value_t = 1.0)]
    pub z_scale: f32,
    #[arg(long, value_delimiter = ',')]
    pub ijk: Option<Vec<usize>>,
    #[arg(short, long)]
    pub property: String,
}

//  //  //  //  //  //  //  //
pub fn parse_cli() -> CliArgs {
    CliArgs::parse()
}
/*
pub fn parse_n_load() -> (gs_loader::LoadedData3D, data3d::SelectedProperty, f32) {
    let cli = CliArgs::parse();

    let gs_loaded = gs_loader::LoadedData3D
        ::from_filename(&cli.property, &cli.ijk)
            .unwrap_or_else(|err| panic!("Fatal: {}", err));
    let selected = data3d::SelectedProperty::new(&gs_loaded, cli.property_index, UNDEF);


    (
        gs_loaded,
        selected,
        cli.z_scale,
    )
}
*/
