use gnuplot::*;
use std::cmp::min;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::process;
use std::thread;

pub mod structs;
pub mod enums;


pub fn generate_gif(plots : &Vec<structs::PlotParameters>, gif : &structs::GifParameters) -> Result<(), Box<dyn Error>> {
    match fs::remove_dir_all("function-to-gif-temp-dir/") {
        Err(clear_dir_error) if clear_dir_error.kind() != ErrorKind::NotFound => {
            return Err(Box::new(clear_dir_error));
        },
        _ => {},
    }

    fs::create_dir_all("function-to-gif-temp-dir/")?;

    let threads_count : usize =
        match thread::available_parallelism() {
            Ok(count) => {
                count.get()
            },
            _ => {
                1
            },
        };

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

    let mut svg_to_png = process::Command::new("ffmpeg");
    svg_to_png.args(&["-y",
                      "-loglevel",
                      "error",
                      "-i",
                      "function-to-gif-temp-dir/plot%d.svg",
                      "-vf",
                      format!("split[bg][fg];[bg]drawbox=c={}@1:replace=1:t=fill[bg];[bg][fg]overlay=format=auto", gif.background_color).as_str(),
                      "function-to-gif-temp-dir/plot%d.png",
    ]).output()?;

    let mut png_to_gif = process::Command::new("ffmpeg");
    png_to_gif.args(&["-y",
                      "-loglevel",
                      "error",
                      "-framerate",
                      format!("{}", gif.fps).as_str(),
                      "-i",
                      "function-to-gif-temp-dir/plot%d.png",
                      "-vf",
                      "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
                      gif.output_file_name,
    ]).output()?;

    fs::remove_dir_all("function-to-gif-temp-dir/")?;

    return Ok(());
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

    let start = plots.iter().min_by(|a, b| (a.x_start).partial_cmp(&b.x_start).unwrap()).unwrap().x_start;
    let end = plots.iter().max_by(|a, b| (a.x_end).partial_cmp(&b.x_end).unwrap()).unwrap().x_end;

    axes.set_x_range(gnuplot::Fix(start), gnuplot::Fix(end));

    for i in 0..plots.len() {
        xs.push(Vec::new());
        ys.push(Vec::new());
        let expr : meval::Expr = plots[i].function.parse().unwrap();
        let f = expr.bind2("x", "t").unwrap();
        let mut x : f64 = plots[i].x_start;

        while x < plots[i].x_end + plots[i].function_step {
            xs[i].push(x);
            ys[i].push(f(x, k as f64 * plots[i].offset_by_frame));
            x += plots[i].function_step;
        }

        match plots[i].plot_type {
            enums::PlotType::Lines {line_width} => {
                axes.lines(&xs[i], &ys[i], &[LineWidth(line_width),
                                             Color(plots[i].color)]);
            }
            enums::PlotType::Points {point_size, point_symbol} => {
                axes.points(&xs[i], &ys[i], &[PointSize(point_size),
                                              PointSymbol(point_symbol),
                                              Color(plots[i].color)]);
            }
            enums::PlotType::LinesPoints {line_width, point_size, point_symbol} |
            enums::PlotType::PointsLines {line_width, point_size, point_symbol} => {
                axes.lines_points(&xs[i], &ys[i], &[LineWidth(line_width),
                                                    PointSize(point_size),
                                                    PointSymbol(point_symbol),
                                                    Color(plots[i].color)]);
            }
        }

        match plots[i].fill_region {
            enums::FillRegion::Above {alpha} => {
                axes.fill_between(&xs[i], (0..xs[i].len()).map(|_| plots[i].max_y).into_iter().collect::<Vec<f64>>(), &ys[i], &[Color(plots[i].color),
                                                                                                                                FillAlpha(alpha),
                                                                                                                                FillRegion(Above)]);
            }
            enums::FillRegion::Below {alpha} => {
                axes.fill_between(&xs[i], (0..xs[i].len()).map(|_| plots[i].min_y).into_iter().collect::<Vec<f64>>(), &ys[i], &[Color(plots[i].color),
                                                                                                                                FillAlpha(alpha),
                                                                                                                                FillRegion(Below)]);
            }
            enums::FillRegion::None => {}
        }

        axes.set_y_range(gnuplot::Fix(plots[i].min_y), gnuplot::Fix(plots[i].max_y));
    }

    fg.save_to_svg(format!("function-to-gif-temp-dir/plot{}.svg", k), gif.width as u32, gif.height as u32).unwrap();
}
