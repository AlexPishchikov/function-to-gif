#[derive(Clone, Copy)]
pub enum FillRegion {
    Above {
        alpha : f64,
    },
    Below {
        alpha : f64,
    },
    None,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlotType {
    Lines {
        line_width : f64,
    },
    Points {
        point_size : f64,
        point_symbol : char,
    },
    LinesPoints {
        line_width : f64,
        point_size : f64,
        point_symbol : char,
    },
    PointsLines {
        line_width : f64,
        point_size : f64,
        point_symbol : char,
    },
}
