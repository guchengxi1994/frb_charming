use charming::ImageRenderer;

use crate::charts::{self, line_chart::LineChartData, BaseChartTrait};

pub fn new_line_chart(title: String, data: LineChartData) -> Option<Vec<u8>> {
    let r = compose(title, data);
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

fn compose(title: String, data: LineChartData) -> anyhow::Result<Vec<u8>> {
    let mut chart = charts::line_chart::LineChart::default();
    chart.data = data;
    chart.title = title;
    let c = chart.compose();
    let mut renderer = ImageRenderer::new(1000, 800);
    let r = renderer.render_format(charming::ImageFormat::Png, &c)?;

    anyhow::Ok(r)
}
