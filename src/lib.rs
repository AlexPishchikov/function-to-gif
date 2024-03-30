use gnuplot::*;
use std::cmp::{min};
use std::fs::{File, create_dir_all, remove_file, remove_dir_all};
use std::io::Write;
use std::process;
use std::thread;

pub mod structs;
pub mod enums;


pub fn generate_gif(plots : &Vec<structs::PlotParameters>, gif : &structs::GifParameters) {
    create_dir_all("plots/");

    let threads_count : usize = thread::available_parallelism().unwrap().get();

    for k in (0..gif.frames_count).step_by(threads_count) {
        thread::scope(|scope| {
            for i in 0..min(threads_count, gif.frames_count - k) {
                scope.spawn(move || {
                    generate_frame(plots, gif, k + i);
                });
            }
        });
    }

    println!("frames generated");

    let mut cmd = process::Command::new("ffmpeg");
    cmd.args(&["-y",
            "-hide_banner",
            "-loglevel",
            "error",
            "-framerate",
            format!("{}", gif.fps).as_str(),
            "-i",
            "plots/plot%d.svg",
            "-filter_complex",
            format!("[0]split=2[bg][fg];[bg]drawbox=c={}@1:replace=1:t=fill[bg];[bg][fg]overlay=format=auto", gif.background_color).as_str(),
            gif.output_file_name
    ]);

    cmd.status().expect("failed to execute process");

    remove_dir_all("plots/");
}

fn generate_frame(plots : &Vec<structs::PlotParameters>, gif : &structs::GifParameters, k : usize) {
    let mut xs : Vec<Vec<f64>> = Vec::new();
    let mut ys : Vec<Vec<f64>> = Vec::new();
    let mut fg = Figure::new();
    let axes = fg.axes2d();
    axes.set_x_grid(false)
        .set_y_grid(false)
        .set_x_axis(false, &[])
        .set_y_axis(false, &[])
        .set_x_ticks(None, &[], &[])
        .set_y_ticks(None, &[], &[])
        .set_border(false, &[], &[])
        .set_margins(&[gnuplot::MarginLeft(0.0), gnuplot::MarginRight(gif.width as f32), gnuplot::MarginTop(0.0), gnuplot::MarginBottom(gif.height as f32)]);

    let max_step = plots.iter().max_by(|a, b| (a.function_step).partial_cmp(&b.function_step).unwrap()).unwrap().function_step;
    let start = plots.iter().min_by(|a, b| (a.x_start).partial_cmp(&b.x_start).unwrap()).unwrap().x_start;
    let end = plots.iter().max_by(|a, b| (a.x_end).partial_cmp(&b.x_end).unwrap()).unwrap().x_end;

    axes.set_x_range(gnuplot::Fix(start), gnuplot::Fix(end));

    for i in 0..plots.len() {
        xs.push(Vec::new());
        ys.push(Vec::new());
        let expr : meval::Expr = plots[i].function.parse().unwrap();
        let f = expr.bind2("x", "t").unwrap();
        let mut x : f64 = plots[i].x_start;

        while x < plots[i].x_end + max_step {
            xs[i].push(x);
            ys[i].push(f(x, k as f64 * plots[i].offset_by_frame));
            x += plots[i].function_step;
        }

        if plots[i].plot_type == enums::PlotType::Lines {
            axes.lines(&xs[i], &ys[i], &[LineWidth(plots[i].line_width), Color(plots[i].color)]);
        }
        if plots[i].plot_type == enums::PlotType::Points {
            axes.points(&xs[i], &ys[i], &[PointSize(plots[i].point_size), PointSymbol(plots[i].point_symbol), Color(plots[i].color)]);
        }
        if plots[i].plot_type == enums::PlotType::LinesPoints || plots[i].plot_type == enums::PlotType::PointsLines {
            axes.lines_points(&xs[i], &ys[i], &[LineWidth(plots[i].line_width), PointSize(plots[i].point_size), PointSymbol(plots[i].point_symbol), Color(plots[i].color)]);
        }

        axes.set_y_range(gnuplot::Fix(plots[i].min_y), gnuplot::Fix(plots[i].max_y));
    }

    let save_plot = fg.save_to_svg(format!("plots/plot{}.svg", k), gif.height as u32, gif.width as u32);
    match save_plot {
        Ok(_) => {},
        Err(save_plot) => {
            eprintln!("{:?}", save_plot);
            process::exit(1);
        },
    }
}
