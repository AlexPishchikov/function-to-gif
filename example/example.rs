extern crate function_to_gif;
use function_to_gif::{structs::GifParameters, structs::PlotParameters, generate_gif};


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
        height : 830,
        fps : 50,
        frames_count : (209 - 33) / 2, // set «(x_end - x_start - 1) * 1 / offset_by_frame» for perfect loop for periodic functions with period equals «(x_end - x_start)»
        background_color : "#0D1117",
        output_file_name : "../example0.gif"
    };

    generate_gif(&plots, &gif);


    let plots = PlotParameters {
        function : vec![
                        "-(sin(2 * x + t) - 2 * sin(2 * x - 5) + cos(5 * x + t)) - 8",
                        "-(sin(2 * x + t)^2 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)) - 8",
                        "sin(2 * x + t) - 2 * sin(2 * x - 5) + cos(5 * x + t)",
                        "sin(2 * x + t)^2 - 2 * sin(2 * x - 5)^3 + cos(5 * x + t)",
        ],
        line_color : vec![
                        "#820263",
                        "#E63946",
                        "#E63946",
                        "#820263",
        ],
        lines_width : 2.0,
        function_step : 0.01,
        offset_by_frame : 0.1,
        x_start : 0.0,
        x_end : 6.3,
        min_y : -12.4,
        max_y : 4.4,
    };

    let gif = GifParameters {
        width : 200,
        height : 830,
        fps : 50,
        frames_count : -((2.0 - 8.3) * 20.0) as i32,
        background_color : "#0D1117",
        output_file_name : "../example1.gif"
    };

    generate_gif(&plots, &gif);
}
