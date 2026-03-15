mod colors;
pub use colors::*;

pub mod svg;
pub use svg::*;

use std::f64::consts::PI;


pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Svg {
    width: f64,
    height: f64,
    elements: Vec<String>,
}

impl Svg {
    pub fn new(w: f64, h: f64) -> Self {
        Self {
            width: w,
            height: h,
            elements: Vec::new(),
        }
    }
    pub fn push_raw(&mut self, s: &str) {
        self.elements.push(s.to_string());
    }
    pub fn finish(self) -> String {
        format!(
            r#"<svg width="{0}" height="{1}" viewBox="0 0 {0} {1}" xmlns="http://www.w3.org/2000/svg">{2}</svg>"#,
            self.width,
            self.height,
            self.elements.join("\n")
        )
    }
}

pub struct Polygon {
    pub points: String,
    pub style: String,
}
impl Polygon {
    pub fn render(&self, svg: &mut Svg) {
        svg.push_raw(&format!(
            r#"<polygon points="{}" style="{}" />"#,
            self.points, self.style
        ));
    }
}

pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub style: String,
}
impl Line {
    pub fn render(&self, svg: &mut Svg) {
        svg.push_raw(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="{}" />"#,
            self.x1, self.y1, self.x2, self.y2, self.style
        ));
    }
}

pub struct Text {
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub style: String,
}
impl Text {
    pub fn render(&self, svg: &mut Svg) {
        svg.push_raw(&format!(
            r#"<text x="{}" y="{}" style="{}">{}</text>"#,
            self.x, self.y, self.style, self.content
        ));
    }
}

pub struct Series {
    pub label: String,
    pub values: Vec<f64>,
    pub color: String,
}

pub struct RadarConfig {
    pub size: f64,
    pub axes_labels: Vec<String>,
    pub padding: f64,
}

pub struct RadarLayout {
    pub center: Point,
    pub radius: f64,
}

pub struct RadarChart {
    pub config: RadarConfig,
    pub layout: RadarLayout,
    pub data: Vec<Series>,
    svg: Svg,
}

impl RadarChart {
    pub fn new(c: RadarConfig) -> Self {
        let center_val = c.size / 2.0;
        let radius_val = center_val - c.padding;

        Self {
            layout: RadarLayout {
                center: Point {
                    x: center_val,
                    y: center_val,
                },
                radius: radius_val,
            },
            svg: Svg::new(c.size, c.size),
            config: c, // Use the passed config
            data: vec![],
        }
    }

    /// Calculate the angle for a specific axis index
    /// Spacing is 2π / N, offset by -π/2 to start at the top (12 o'clock)
    pub fn axis_angle(&self, i: usize, total_axes: usize) -> f64 {
        (2.0 * PI * i as f64 / total_axes as f64) - (PI / 2.0)
    }

    pub fn value_point(&self, axis: usize, total_axes: usize, value: f64) -> Point {
        let angle = self.axis_angle(axis, total_axes);
        let r = self.layout.radius * value;

        Point {
            x: self.layout.center.x + r * angle.cos(),
            y: self.layout.center.y + r * angle.sin(),
        }
    }

    pub fn polygon_points(&self, values: &[f64]) -> String {
        let n = values.len();
        values
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let p = self.value_point(i, n, *v);
                format!("{:.2},{:.2}", p.x, p.y)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn edge_point(&self, axis: usize, total_axes: usize) -> Point {
        self.value_point(axis, total_axes, 1.0)
    }

    fn render_legend(&mut self) {
        let x_start = 10.0;
        let mut y_start = 10.0;
        let spacing = 18.0;

        for series in &self.data {
            // Colored indicator (Square)
            self.svg.push_raw(&format!(
                r#"<rect x="{}" y="{}" width="08" height="08" rx="2" fill="{}" />"#,
                x_start, y_start, series.color
            ));

            // Series Label
            Text {
                x: x_start + 15.0,
                y: y_start + 06.7,
                content: series.label.clone(),
                style: format!(
                    "fill:{};font-size:08px;font-family:sans-serif;font-weight:bold;",
                    Mocha::TEXT
                ),
            }
            .render(&mut self.svg);

            y_start += spacing;
        }
    }

    fn draw_axis_and_label(&mut self, n: usize) {
        // 3. Axis Lines & Labels
        for i in 0..n {
            let edge = self.edge_point(i, n);

            // Axis Line
            Line {
                x1: self.layout.center.x,
                y1: self.layout.center.y,
                x2: edge.x,
                y2: edge.y,
                style: format!(
                    "stroke:{};stroke-opacity:0.3;stroke-dasharray:2;",
                    Mocha::SUBTEXT0
                ),
            }
            .render(&mut self.svg);

            // Label Positioning
            if let Some(label) = self.config.axes_labels.get(i) {
                let angle = self.axis_angle(i, n);
                let cos_val = angle.cos();

                // Offset label slightly outside the radius
                let r_label = self.layout.radius + 15.0;
                let lx = self.layout.center.x + r_label * cos_val;
                let ly = self.layout.center.y + r_label * angle.sin();

                let anchor = if cos_val.abs() < 0.1 {
                    "middle"
                } else if cos_val > 0.0 {
                    "start"
                } else {
                    "end"
                };

                Text {
                    x: lx, y: ly,
                    content: label.clone(),
                    style: format!("fill:{};font-size:10px;text-anchor:{};dominant-baseline:middle;font-family:sans-serif;", Mocha::TEXT, anchor),
                }.render(&mut self.svg);
            }
        }
    }

    fn draw_grid(&mut self, n: usize) {
        // 2. Grid Polygons (Concentric Webs)
        for level in 1..=4 {
            let scale = level as f64 / 4.0;
            Polygon {
                points: self.polygon_points(&vec![scale; n]),
                style: format!("fill:none;stroke:{};stroke-opacity:0.2;", Mocha::SUBTEXT0),
            }
            .render(&mut self.svg);
        }
    }

    fn plot_data(&mut self, n: usize) {
        // Data Polygons
        for series in &self.data {
            let initial_points = self.polygon_points(&vec![0.0; n]); // Points at center
            let target_points = self.polygon_points(&series.values);

            self.svg.push_raw(&format!(
    r###"<polygon points="{target_points}" style="fill:{color};fill-opacity:0.25;stroke:{color};stroke-width:1;">
        <animate 
            attributeName="points" 
            from="{initial_points}" 
            to="{target_points}" 
            dur="1.8s" 
            begin="0s" 
            fill="freeze" 
            calcMode="spline" 
            keyTimes="0;1" 
            keySplines="0.4 0 0.2 1" />
    </polygon>"###,
    target_points = target_points,
    initial_points = initial_points,
    color = series.color
));
        }
    }

    fn draw_bg(&mut self) {
        self.svg.push_raw(&format!(
            r#"<rect width="100%" height="100%" fill="{}" />"#,
            Mocha::CRUST
        ));
    }

    pub fn render(mut self) -> String {
        let n = self.data.first().map(|s| s.values.len()).unwrap_or(0);
        if n == 0 {
            return self.svg.finish();
        }

        self.draw_bg();
        self.render_legend();
        self.draw_grid(n);
        self.draw_axis_and_label(n);
        self.plot_data(n);

        self.svg.finish()
    }

    pub fn set_data(&mut self, data: Vec<Series>) {
        self.data = data;
    }
}


