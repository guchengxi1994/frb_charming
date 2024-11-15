pub use chart_core::charts::line_chart::LineChartData;
pub use chart_core::config::*;
use flutter_rust_bridge::frb;

#[frb(mirror(LineChartData))]
pub struct MLineChartData {
    pub x: Vec<String>,
    pub y: Vec<f32>,
}

#[frb(mirror(ChartOutputConfig))]
pub struct MChartOutputConfig {
    pub output_size: (u32, u32),
    pub output_format: OutputFormat,
    pub output_path: Option<String>,
}

#[frb(mirror(Output))]
pub enum MOutput {
    File(String),
    Memory(Vec<u8>),
    None,
}

#[frb(mirror(OutputFormat))]
pub enum MOutputFormat {
    Png,
    Svg,
}

pub fn new_line_chart(title: String, data: LineChartData) -> Option<Vec<u8>> {
    chart_core::new_line_chart(title, data)
}

pub fn new_line_chart_with_config(
    title: String,
    data: LineChartData,
    config: ChartOutputConfig,
) -> Output {
    chart_core::new_line_chart_with_config(title, data, config)
}
