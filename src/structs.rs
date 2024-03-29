use crate::enums::PlotType;

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
    pub line_width : f64,
    pub point_size : f64,
    pub point_symbol : char,
    pub function_step : f64,
    pub offset_by_frame : f64,
    pub x_start : f64,
    pub x_end : f64,
    pub min_y : f64,
    pub max_y : f64,
}
