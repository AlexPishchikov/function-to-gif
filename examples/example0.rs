extern crate function_to_gif;
use function_to_gif::{enums::{FillRegion, PlotType}, structs::{GifParameters, PlotParameters}, generate_gif};


fn main() {
    let functions : Vec<&str> = vec![
        "ceil(-(sin(2 * x + t) - 2 * sin(2 * x - 5) + cos(5 * x + t)) + 6)",
        "ceil(-(sin(2 * x + t)^2 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)) + 3)",
        "ceil(-(sin(2 * x + t)^3 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)))",
    ];

    let line_colors : Vec<&str> = vec![
        "#009DDC",
        "#820263",
        "#E63946",
    ];

    let fill : Vec<FillRegion> = vec![
        FillRegion::Below {alpha : 0.5},
        FillRegion::Below {alpha : 0.6},
        FillRegion::Below {alpha : 0.8},
    ];

    let line_widths : Vec<f64> = vec![
        2.0,
        0.0,
        0.0,
    ];

    let mut plots : Vec<PlotParameters> = Vec::new();

    for i in 0..functions.len() {
        let plot = PlotParameters {
            function : functions[i],
            color : line_colors[i],
            plot_type : PlotType::Lines { line_width : line_widths[i] },
            fill_region : fill[i],
            function_step : 0.04,
            offset_by_frame : 0.1,
            x_start : 0.0,
            x_end : 6.3,
            min_y : -4.4,
            max_y : 10.4,
        };
        plots.push(plot);
    }

    let gif = GifParameters {
        width : 830,
        height : 200,
        fps : 50,
        frames_count : ((plots[0].x_end - plots[0].x_start) * 10.0) as usize - 1,
        background_color : "#0D1117",
        output_file_name : "example0.gif"
    };

    generate_gif(&plots, &gif).unwrap();
}
