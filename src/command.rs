use clap::Parser;

use properties3d::*;
use properties3d::types3d::*;

use crate::data;

const U: &str = "-999";

//  //  //  //  //  //  //  //
pub fn run() -> (data::Data3D, f32) {
    let cli = CliArgs::parse();

    let grid = CachedGrid::new(cli.i_max, cli.j_max, cli.k_max);
    let property = Property::<Continuous>::from_file(&cli.property, grid.size(), U)
            .unwrap_or_else(|err| panic!("{}", err));

    let mut min_value: Option<f32> = None;
    let mut max_value: Option<f32> = None;
    let mut sum_num: Option<(f32, usize)> = None;
    for index in 0..grid.size() {
        if let Some(v) = property[index] {
            let v = v as f32;
            if min_value == None {
                min_value = Some(v);
            } else {
                if v < min_value.unwrap() {
                    min_value = Some(v);
                }
            }
            if max_value == None {
                max_value = Some(v);
            } else {
                if v > max_value.unwrap() {
                    max_value = Some(v);
                }
            }
            if let Some((sum, num)) = sum_num {
                sum_num = Some((sum+v, num+1));
            } else {
                sum_num = Some((v, 1));
            }
        }
    }

    let min_value = min_value
            .expect("undefined min_value");
    let max_value = 1e-6 + max_value
            .expect("undefined max_value");
    let (sum, num) = sum_num
            .expect("no values to average");
    let avr_value = sum / (num as f32);

    (
        data::Data3D {
            name: cli.property,
            grid,
            property,
            min_value,
            avr_value,
            max_value,
        },
        cli.z_scale,
    )
}

//  //  //  //  //  //  //  //
#[derive(clap::Parser, Debug)]
#[command(about)]
struct CliArgs {
    #[arg(short, long, default_value_t=1.0)]
    z_scale: f32,
    #[arg(short, long)]
    i_max: usize,
    #[arg(short, long)]
    j_max: usize,
    #[arg(short, long)]
    k_max: usize,
    #[arg(short, long)]
    property: String,
}
