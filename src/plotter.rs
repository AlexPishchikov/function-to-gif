use gnuplot::*;
use meval::Expr;
use std::process;
use std::io::Write;
use std::fs::{File, create_dir_all, remove_file, remove_dir_all};

pub struct PlotParameters<'a> {
    pub function : Vec<&'a str>,
    pub line_color : Vec<&'a str>,
    pub lines_width : f64,
    pub function_step : f64,
    pub offset_by_frame : f64,
    pub x_start : f64,
    pub x_end : f64,
    pub min_y : f64,
    pub max_y : f64,
}

pub struct GifParameters<'a> {
    pub width : u32,
    pub height : u32,
    pub fps : u32,
    pub frames_count : i32,
    pub background_color : &'a str,
    pub output_file_name : &'a str,
}

pub fn draw(plots : &PlotParameters, gif : &GifParameters) {
    let mut list = File::create("plots_list.txt").expect("error");
    create_dir_all("plots/");

    for k in 0..gif.frames_count {
        let mut xs : Vec<f64> = Vec::new();
        let mut ys : Vec<Vec<f64>> = Vec::new();
        let mut fg = Figure::new();

        for i in 0..plots.function.len() {
            ys.push(Vec::new());
            let expr : meval::Expr = plots.function[i].parse().unwrap();
            let f = expr.bind2("x", "t").unwrap();
            let mut x : f64 = plots.x_start;

            while x < plots.x_end {
                xs.push(x);
                ys[i].push(f(x, k as f64 * plots.offset_by_frame));
                x += plots.function_step;
            }

            fg.axes2d().lines(&xs, &ys[i], &[LineWidth(plots.lines_width), Color(plots.line_color[i])])
                .set_y_range(gnuplot::Fix(plots.min_y), gnuplot::Fix(plots.max_y))
                .set_x_grid(false)
                .set_y_grid(false)
                .set_x_axis(false, &[])
                .set_y_axis(false, &[])
                .set_x_ticks(None, &[], &[])
                .set_y_ticks(None, &[], &[])
                .set_border(false, &[], &[])
                .set_margins(&[gnuplot::MarginLeft(0.0), gnuplot::MarginRight(gif.width as f32), gnuplot::MarginTop(0.0), gnuplot::MarginBottom(gif.height as f32)]);
        }

        let save_plot = fg.save_to_svg(format!("plots/plot{}.svg", k), gif.height, gif.width);
        match save_plot {
            Ok(_) => {},
            Err(save_plot) => {
                eprintln!("{:?}", save_plot);
                process::exit(1);
            },
        }
        writeln!(&mut list, "plots/plot{}.svg", k);
    }

    println!("{}", "frames generated");

    process::Command::new("convert")
        .arg("-delay")
        .arg(format!("1x{}", gif.fps))
        .arg("-background")
        .arg(format!("{}", gif.background_color))
        .arg("-loop")
        .arg("0")
        .arg("-dispose")
        .arg("previous")
        .arg("@plots_list.txt")
        .arg(format!("{}", gif.output_file_name))
        .status()
        .expect("failed to execute process");

    remove_file("plots_list.txt");
    remove_dir_all("plots");
}
