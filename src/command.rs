use clap::Parser;

use properties3d::*;
use properties3d::types3d::*;

const U: &str = "-999";



//  //  //  //  //  //  //  //
pub fn run() -> Data3D {
    let cli = CliArgs::parse();

    let grid = CachedGrid::new(cli.i_max, cli.j_max, cli.k_max);
    let property = Property::<Continuous>::from_file(&cli.property, grid.size(), U)
            .unwrap_or_else(|err| panic!("{}", err));

    let mut min_value: Option<f32> = None;
    let mut max_value: Option<f32> = None;
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
        }
    }

    let min_value = min_value
            .expect("undefined min_value");
    let max_value = 1e-9 + max_value
            .expect("undefined max_value");

    Data3D {
        name: cli.property,
        grid,
        property,
        min_value,
        max_value,
    }
}

pub struct Data3D {
    pub name: String,
    pub grid: CachedGrid,
    pub property: Property<Continuous>,
    pub min_value: f32,
    pub max_value: f32,
}
impl Data3D {
    pub fn get_norm_value(&self, i: usize, j: usize, k: usize) -> Option<f32> {
        let ijk = IJK {
            i: i,
            j: j,
            k: k,
        };

        let Some(index) = self.grid.index_from(&ijk) else {
            return None;
        };
        let Some(v) = self.property[index] else {
            return None;
        };
        Some( ((v as f32)-self.min_value)/(self.max_value-self.min_value) )
    }
}


//  //  //  //  //  //  //  //
#[derive(clap::Parser, Debug)]
#[command(about)]
struct CliArgs {
    #[arg(short, long, default_value_t=37)]
    i_max: usize,
    #[arg(short, long, default_value_t=43)]
    j_max: usize,
    #[arg(short, long, default_value_t=13)]
    k_max: usize,
    #[arg(short, long)]
    property: String,
}
