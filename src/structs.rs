pub struct GifParameters<'a> {
    pub width : u32,
    pub height : u32,
    pub fps : u32,
    pub frames_count : i32,
    pub background_color : &'a str,
    pub output_file_name : &'a str,
}

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
