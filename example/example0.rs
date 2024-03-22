use std::f64::consts::PI;

extern crate function_to_gif;
use function_to_gif::{enums::PlotType, structs::GifParameters, structs::PlotParameters, generate_gif};


fn main() {
    let functions : Vec<&str> = vec![
        "2.9 * sin(x / 10) * (sin(pi * (x + t) - 3) + cos(-3 * (x + t) + pi))",
        "6 * abs(cos((x + t - 10) / 14.1 + pi))",
        "-6 * abs(cos((x + t - 10) / 14.1))",
        "6 * abs(cos((x + t - 10) / 14.1 + pi)) * sin(x / 10)",
        "-6 * abs(cos((x + t - 10) / 14.1)) * sin(x / 10)",
    ];

    let line_colors : Vec<&str> = vec![
        "#8E1C72",
        "#E63946",
        "#7BDA3C",
        "#FFA400",
        "#0FA3B1",
    ];

    let plot_types : Vec<PlotType> = vec![
        PlotType::Lines,
        PlotType::Points,
        PlotType::Points,
        PlotType::PointsLines,
        PlotType::PointsLines,
    ];

    let sizes : Vec<(f64, f64)> = vec![
        (2.0, 0.0),
        (0.0, 1.0),
        (0.0, 1.0),
        (2.0, 0.8),
        (2.0, 0.8),
    ];

    let steps : Vec<f64> = vec![
        0.1,
        1.7,
        1.7,
        2.2,
        2.2,
    ];

    let mut plots : Vec<PlotParameters> = Vec::new();

    for i in 0..functions.len() {
        let plot = PlotParameters {
            function : functions[i],
            color : line_colors[i],
            plot_type : plot_types[i],
            line_width : sizes[i].0,
            point_size : sizes[i].1,
            point_symbol : 'r',
            function_step : steps[i],
            offset_by_frame : 0.5,
            x_start : 0.0,
            x_end : 50.0 * PI,
            min_y : -6.8,
            max_y : 6.8,
        };
        plots.push(plot);
    }

    let gif = GifParameters {
        width : 100,
        height : 830,
        fps : 50,
        frames_count : ((plots[0].x_end - plots[0].x_start) / (3.57 * plots[0].offset_by_frame)) as usize, // set «(x_end - x_start - 1) * 1 / offset_by_frame»
                                                                                                           // for perfect loop for periodic functions with period equals «(x_end - x_start)»
        background_color : "#0D1117",
        output_file_name : "../example0.gif"
    };

    generate_gif(&plots, &gif);
}
