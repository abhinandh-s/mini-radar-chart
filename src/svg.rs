#[derive(Clone)]
pub struct Svg(String);

pub type StyleElm<'a> = (&'a str, &'a str);

impl AsRef<[u8]> for Svg {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// "<line x1="{sx}" y1="{sy}" x2="{ex}" y2="{ey}" stroke="#aaa"/>"
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub style: String,
}

impl RenderSvg for Line {
    fn render(&self, svg: &mut Svg) {
        write!(
            svg,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2, self.style
        )
        .unwrap();
    }
}

pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub style: String,
}

impl Circle {
    pub fn cx(&self) -> (f64, f64) {
        (self.cx, self.cy)
    }

    /// Converts a value (0.0 to 1.0) on a specific axis to a Point
    pub fn point_at(&self, axis_idx: usize, axes_count: usize, scale: f64) -> (f64, f64) {
        // -PI/2 rotates the chart so the first axis is at the top
        let angle = (2.0 * PI * axis_idx as f64 / axes_count as f64) - (PI / 2.0);
        let x = self.cx + self.r * scale * angle.cos();
        let y = self.cy + self.r * scale * angle.sin();
        (x, y)
    }
}

pub struct Text {
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub style: String,
}

impl RenderSvg for Text {
    fn render(&self, svg: &mut Svg) {
        write!(
            svg,
            r#"<text x="{}" y="{}" style="{}" font-family="sans-serif">{}</text>"#,
            self.x, self.y, self.style, self.content
        )
        .unwrap();
    }
}

pub struct Polygon {
    pub points: String,
    pub style: String,
}

impl Polygon {
    // where:
    //  value = 0.0..1.0
    pub fn points_from_value(&self, _values: &[f64], attributes: &str) -> String {
        let points = "w";
        format!(r#"<polygon points="{}" {} />"#, points, attributes)
    }
}

impl RenderSvg for Polygon {
    fn render(&self, svg: &mut Svg) {
        write!(
            svg,
            "<polygon points=\"{p}\" style=\"{s}\" />",
            p = self.points,
            s = self.style,
        )
        .unwrap();
    }
}

impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

use std::f64::consts::PI;
use std::fmt::Write;

impl std::fmt::Write for Svg {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}

pub trait RenderSvg {
    fn render(&self, svg: &mut Svg);
}

impl RenderSvg for Circle {
    fn render(&self, svg: &mut Svg) {
        write!(
            svg,
            r#"<circle cx="{}" cy="{}" r="{}" style="{}" />"#,
            self.cx, self.cy, self.r, self.style
        )
        .unwrap();
    }
}

impl Svg {
    pub fn new(h: f64, w: f64) -> Self {
        Self(format!(
            r###"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">"###
        ))
    }

    pub fn push_raw(&mut self, string: &str) -> &mut Self {
        self.0.push_str(string);
        self
    }

    pub fn finish(&mut self) -> String {
        self.push_raw("</svg>");
        std::mem::take(&mut self.0)
    }
}
