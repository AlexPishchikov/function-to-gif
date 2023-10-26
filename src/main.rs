mod plotter;
use plotter::{PlotParameters, GifParameters, draw};

fn main() {
    let plots = PlotParameters {
        function : vec![
                        "2.9 * sin(x / 10) * (sin(pi * (x + t) - 3) + cos(-3 * (x + t) + pi))",
                        "6 * abs(cos((x + t - 10) / 14.1 + pi))",
                        "-6 * abs(cos((x + t - 10) / 14.1))",
                        "6 * abs(cos((x + t - 10) / 14.1 + pi)) * sin(x / 10)",
                        "-6 * abs(cos((x + t - 10) / 14.1)) * sin(x / 10)",
        ],
        line_color : vec![
                        "#820263",
                        "#E63946",
                        "#7BDA3C",
                        "#FFA400",
                        "#0FA3B1",
        ],
        lines_width : 2.0,
        function_step : 0.1,
        offset_by_frame : 0.5,
        x_start : 32.0,
        x_end : 209.0,
        min_y : -6.8,
        max_y : 6.8,
    };

    let gif = GifParameters {
        width : 100,
        height : 500,
        fps : 50,
        frames_count : (209 - 33) / 2, // set «(x_end - x_start - 1) * 1 / offset_by_frame» for perfect loop for periodic functions with period equals «(x_end - x_start)»
        background_color : "#000000",
        output_file_name : "example.gif"
    };

    draw(&plots, &gif);
}
