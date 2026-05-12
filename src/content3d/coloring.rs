use kiss3d::prelude::*;

//  //  //  //  //  //  //  //
pub(crate) fn calculate_color(v: f32) -> Color {
    let colors = [
        Color::new( 0.4, 0.0, 1.0, 1.0 ),
        Color::new( 0.1, 0.1, 1.0, 1.0 ),
        Color::new( 0.0, 1.0, 0.0, 1.0 ),
        Color::new( 1.0, 1.0, 0.0, 1.0 ),
        Color::new( 1.0, 0.0, 0.0, 1.0 ),
    ];
    match v {
        0.0..0.25 => {
            mix_colors( v*4., &colors[0], &colors[1] )
        },
        0.25..0.50 => {
            mix_colors( (v-0.25)*4., &colors[1], &colors[2] )
        },
        0.50..0.75 => {
            mix_colors( (v-0.50)*4., &colors[2], &colors[3] )
        },
        _ => {
            mix_colors( (v-0.75)*4., &colors[3], &colors[4] )
        },
    }
}

pub fn mix_colors( w: f32, min_color: &Color, max_color: &Color) -> Color {
    let uw = 1.-w*w;
    let uw = 1.-uw;
    let uw = 1.-uw*uw;
    Color::new(
        uw*min_color.r + w*max_color.r,
        uw*min_color.g + w*max_color.g,
        uw*min_color.b + w*max_color.b,
        uw*min_color.a + w*max_color.a,
    )
}

