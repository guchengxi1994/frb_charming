pub mod charts;
pub mod config;

use charming::ImageRenderer;
use charts::{
    bar_chart::{BarChart, BarChartData},
    graph_chart::{GraphChart, GraphChartData, MindGraphChart},
    line_chart::LineChart,
    ChartData,
};
pub use config::Output;

use crate::charts::{line_chart::LineChartData, BaseChartTrait};

pub fn compose<T: ChartData>(
    chart_type: String,
    title: String,
    data: T,
    width: Option<u32>,
    height: Option<u32>,
    format: Option<config::OutputFormat>,
) -> anyhow::Result<Vec<u8>> {
    let mut chart: Box<dyn BaseChartTrait> = match chart_type.as_str() {
        "line" => {
            let mut line_chart = LineChart::default();
            if let Some(d) = data.as_chart_data().downcast_ref::<LineChartData>() {
                line_chart.data = d.clone();
            } else {
                return Err(anyhow::anyhow!("Invalid data type for line chart"));
            }
            Box::new(line_chart)
        }
        "bar" => {
            let mut bar_chart = BarChart::default();
            if let Some(d) = data.as_chart_data().downcast_ref::<BarChartData>() {
                bar_chart.data = d.clone();
            } else {
                return Err(anyhow::anyhow!("Invalid data type for bar chart"));
            }
            Box::new(bar_chart)
        }
        "graph" => {
            let mut graph_chart = GraphChart::default();
            if let Some(d) = data.as_chart_data().downcast_ref::<GraphChartData>() {
                graph_chart.data = d.clone();
            } else {
                return Err(anyhow::anyhow!("Invalid data type for graph chart"));
            }
            Box::new(graph_chart)
        }
        "mind_graph" => {
            let mut mind_graph_chart = MindGraphChart::default();
            if let Some(d) = data.as_chart_data().downcast_ref::<GraphChartData>() {
                mind_graph_chart.data = d.clone();
            } else {
                return Err(anyhow::anyhow!("Invalid data type for mind graph chart"));
            }
            Box::new(mind_graph_chart)
        }
        _ => return Err(anyhow::anyhow!("Chart type not supported")),
    };

    chart.set_title(title);
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
