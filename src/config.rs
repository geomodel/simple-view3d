use clap::Parser;

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
