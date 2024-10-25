use crate::enums::{FillRegion, PlotType};

pub struct GifParameters<'a> {
    pub width : usize,
    pub height : usize,
    pub fps : usize,
    pub frames_count : usize,
    pub background_color : &'a str,
    pub output_file_name : &'a str,
}

pub struct PlotParameters<'a> {
    pub function : &'a str,
    pub color : &'a str,
    pub plot_type : PlotType,
    pub fill_region : FillRegion<'a>,
    pub function_step : f64,
    pub offset_by_frame : f64,
    pub x_start : f64,
    pub x_end : f64,
    pub min_y : f64,
    pub max_y : f64,
}
