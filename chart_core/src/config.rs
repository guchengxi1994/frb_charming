#[derive(Debug,Clone)]
pub enum OutputFormat {
    Png,
    Svg,
}

#[derive(Debug)]
pub enum Output {
    File(String),
    Memory(Vec<u8>),
    None,
}

pub struct ChartOutputConfig {
    pub output_size: (u32, u32),
    pub output_format: OutputFormat,
    pub output_path: Option<String>,
}

impl Default for ChartOutputConfig {
    fn default() -> Self {
        Self {
            output_size: (800, 600),
            output_format: OutputFormat::Png,
            output_path: None,
        }
    }
}
