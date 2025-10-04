use plotters::prelude::*;
use std::error::Error;

// Drawing function using plotters
pub fn draw_dots(
    bitmatrix: &ndarray::Array3<i8>,
    _grid_size: f64,
    filename: &str,
) -> Result<(), Box<dyn Error>> {

    let root_area = BitMapBackend::new(filename, (800, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Anoto Dots", ("sans-serif", 40))
        .build_cartesian_2d(-10_i32..170_i32, 100_i32..-10_i32)
        .unwrap();

    ctx.configure_mesh()
        .x_labels(18)
        .x_label_formatter(&|v| format!("{}", (v / 10) + 1))
        .y_labels(11)
        .y_label_formatter(&|v| format!("{}", (v / 10) + 1))
        .draw().unwrap();

   // Draw circles based on bitmatrix values
    ctx.draw_series(
        (0..bitmatrix.dim().0).flat_map(|y| {
            (0..bitmatrix.dim().1).map(move |x| {
                let mut x_bit = bitmatrix[[y, x, 0]] as usize;
                let mut y_bit = bitmatrix[[y, x, 1]] as usize;
                let dot_type = x_bit + (y_bit << 1);
                let color = match dot_type {
                    0 => &BLACK, // UP
                    1 => &RED,   // RIGHT
                    2 => &BLUE,  // LEFT
                    3 => &GREEN, // DOWN
                    _ => &BLACK,
                };
                let mut x_x :i32 = x.clone() as i32;
                let mut y_y :i32 = y.clone() as i32;
                match dot_type {
                    0 => { // UP
                        x_x = x_x * 10;
                        y_y = y_y * 10 + 2;
                    }
                    1 => { // RIGHT
                        x_x = x_x * 10 + 2;
                        y_y = y_y * 10;
                    }
                    2 => { // LEFT
                        x_x = (x_x * 10) - 2;
                        y_y = y_y * 10;
                    },
                    3 => { // DOWN
                        x_x = x_x * 10;
                        y_y = (y_y * 10) - 2;
                    },
                    _ => {}
                };

                Circle::new((x_x as i32, y_y as i32), 5, color.filled())
            })
        })
    ).unwrap();

    Ok(())
}
