pub mod charts;
pub mod config;

use charming::ImageRenderer;
pub use config::Output;

use crate::charts::{line_chart::LineChartData, BaseChartTrait};

pub fn new_line_chart(title: String, data: LineChartData) -> Option<Vec<u8>> {
    let r = compose(title, data, None, None, None);
    match r {
        Ok(_r) => {
            return Some(_r);
        }
        Err(_e) => {
            println!("render Line chart Error: {:?}", _e);
            return None;
        }
    }
}

pub fn new_line_chart_with_config(
    title: String,
    data: LineChartData,
    config: config::ChartOutputConfig,
) -> Output {
    let r = compose(
        title,
        data,
        Some(config.output_size.0),
        Some(config.output_size.1),
        Some(config.output_format),
    );
    match r {
        Ok(_r) => {
            if let Some(path) = &config.output_path {
                let s = std::fs::write(path, _r);
                match s {
                    Ok(_) => {
                        return Output::File(path.to_string());
                    }
                    Err(_e) => {
                        println!("write file error: {:?}", _e);
                        return Output::None;
                    }
                }
            }

            return Output::Memory(_r);
        }
        Err(_e) => {
            println!("render Line chart Error: {:?}", _e);
            return Output::None;
        }
    }
}

fn compose(
    title: String,
    data: LineChartData,
    width: Option<u32>,
    height: Option<u32>,
    format: Option<config::OutputFormat>,
) -> anyhow::Result<Vec<u8>> {
    let mut chart = charts::line_chart::LineChart::default();
    chart.data = data;
    chart.title = title;
    let c = chart.compose();
    let mut renderer = ImageRenderer::new(width.unwrap_or(1000), height.unwrap_or(800));
    let r;
    if let Some(f) = format {
        match f {
            config::OutputFormat::Png => {
                r = renderer.render_format(charming::ImageFormat::Png, &c)?;
            }
            config::OutputFormat::Svg => {
                let svg_str = renderer.render(&c)?;
                r = svg_str.as_bytes().to_vec();
            }
        }
    } else {
        r = renderer.render_format(charming::ImageFormat::Png, &c)?;
    }

    anyhow::Ok(r)
}
