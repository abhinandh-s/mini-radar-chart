use std::fs;

use mini_radar_chart::*;

fn main() {
    let portfolio_a = vec![0.8, 0.6, 0.9, 0.5, 0.7, 0.4];
    let portfolio_b = vec![0.6, 0.7, 0.5, 0.8, 0.6, 0.5];
    let portfolio_c = vec![0.1, 0.3, 0.7, 0.2, 0.9, 0.8];

    let mut radar = RadarChart::new(RadarConfig {
        size: 300.0,
        axes_labels: vec![
            "Speed".into(),
            "Power".into(),
            "Agility".into(),
            "Stamina".into(),
            "Range".into(),
            "Will".into(),
        ],
        padding: 40.0, // Space for labels
    });

    radar.set_data(vec![
        Series {
            label: "Alpha".into(),
            values: portfolio_a,
            color: Mocha::RED.to_string(),
        },
        Series {
            label: "Beta".into(),
            values: portfolio_b,
            color: Mocha::BLUE.to_string(),
        },
        Series {
            label: "Theta".into(),
            values: portfolio_c,
            color: Mocha::GREEN.to_string(),
        },
    ]);

    let svg_output = radar.render();
    fs::write("radar.svg", svg_output).expect("Unable to write file");
    println!("radar.svg has been generated.");
}
