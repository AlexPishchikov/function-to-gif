extern crate function_to_gif;
use function_to_gif::{enums::PlotType, structs::GifParameters, structs::PlotParameters, generate_gif};


fn main() {
    let functions : Vec<&str> = vec![
        "-(sin(2 * x + t) - 2 * sin(2 * x - 5) + cos(5 * x + t)) - 8",
        "-(sin(2 * x + t)^2 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)) - 8",
        "sin(2 * x + t) - 2 * sin(2 * x - 5) + cos(5 * x + t)",
        "sin(2 * x + t)^2 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)",
    ];

    let line_colors : Vec<&str> = vec![
        "#820263",
        "#E63946",
        "#E63946",
        "#820263",
    ];

    let mut plots : Vec<PlotParameters> = Vec::new();

    for i in 0..functions.len() {
        let plot = PlotParameters {
            function : functions[i],
            color : line_colors[i],
            plot_type : PlotType::Lines,
            line_width : 2.0,
            point_size : 0.0,
            point_symbol : '.',
            function_step : 0.04,
            offset_by_frame : 0.1,
            x_start : 0.0,
            x_end : 6.3,
            min_y : -12.4,
            max_y : 4.4,
        };
        plots.push(plot);
    }

    let gif = GifParameters {
        width : 200,
        height : 830,
        fps : 50,
        frames_count : ((plots[0].x_end - plots[0].x_start) * 10.0) as usize - 1,
        background_color : "#0D1117",
        output_file_name : "../example1.gif"
    };

    generate_gif(&plots, &gif);
}
