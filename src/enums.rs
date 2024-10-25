#[derive(Clone, Copy)]
pub enum FillRegion<'a> {
    Above {
        alpha : f64,
    },
    AboveWithColor {
        alpha : f64,
        color : &'a str,
    },
    Below {
        alpha : f64,
    },
    BelowWithColor {
        alpha : f64,
        color : &'a str,
    },
    BetweenAbove {
        alpha : f64,
        index : usize,
    },
    BetweenAboveWithColor {
        alpha : f64,
        index : usize,
        color : &'a str,
    },
    BetweenBelow {
        alpha : f64,
        index : usize,
    },
    BetweenBelowWithColor {
        alpha : f64,
        index : usize,
        color : &'a str,
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
