use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::Bar,
    Chart,
};

use super::{BaseChart, BaseChartTrait, ChartData};

pub struct BarChart {
    pub base: BaseChart,
    pub data: BarChartData,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct BarChartData {
    pub x: Vec<String>,
    pub y: Vec<f32>,
}

impl BarChartData {
    pub fn new() -> BarChartData {
        BarChartData {
            x: vec![],
            y: vec![],
        }
    }
}

impl ChartData for BarChartData {
    fn as_chart_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

impl BaseChartTrait for BarChart {
    fn default() -> Self {
        BarChart {
            base: BaseChart::new().set_type("line".to_string()),
            data: BarChartData::new(),
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
            .series(Bar::new().data(self.data.y.clone()));
    }
}
