use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::Line,
    Chart,
};

use super::{BaseChart, BaseChartTrait, ChartData};

#[derive(Debug, Clone)]
pub struct LineChartData {
    pub x: Vec<String>,
    pub y: Vec<f32>,
}

impl LineChartData {
    pub fn new() -> LineChartData {
        LineChartData {
            x: vec![],
            y: vec![],
        }
    }
}

impl ChartData for LineChartData {
    fn as_chart_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

pub struct LineChart {
    pub base: BaseChart,
    pub data: LineChartData,
    pub title: String,
}

impl BaseChartTrait for LineChart {
    fn default() -> Self {
        LineChart {
            base: BaseChart::new().set_type("line".to_string()),
            data: LineChartData::new(),
            title: "".to_string(),
        }
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn compose(&self) -> charming::Chart {
        assert!(self.data.x.len() == self.data.y.len());
        return Chart::new()
            .title(Title::new().text(&self.title))
            .x_axis(Axis::new().data(self.data.x.clone()))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(self.data.y.clone()));
    }
}
