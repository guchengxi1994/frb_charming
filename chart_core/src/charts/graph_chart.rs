use charming::{
    component::Legend,
    element::{Label, LabelLayout, LabelPosition, LineStyle, ScaleLimit},
    series::{Graph, GraphData, GraphLayout},
};

use super::{BaseChartTrait, ChartData};

pub struct GraphChart {
    pub base: super::BaseChart,
    pub data: GraphChartData,
    pub title: String,
}

pub struct MindGraphChart {
    pub base: super::BaseChart,
    pub data: GraphChartData,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct GraphChartData {
    pub data: String,
}

impl ChartData for GraphChartData {
    fn as_chart_data(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

impl BaseChartTrait for GraphChart {
    fn default() -> Self
    where
        Self: Sized,
    {
        GraphChart {
            base: super::BaseChart::new().set_type("graph".to_string()),
            data: GraphChartData {
                data: "".to_string(),
            },
            title: "".to_string(),
        }
    }

    fn compose(&self) -> charming::Chart {
        let data: GraphData = serde_json::from_str(&self.data.data).unwrap_or(GraphData {
            nodes: vec![],
            links: vec![],
            categories: vec![],
        });
        charming::Chart::new()
            .legend(Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()))
            .series(
                Graph::new()
                    .name(self.title.clone())
                    .layout(GraphLayout::None)
                    .roam(true)
                    .label(
                        Label::new()
                            .show(true)
                            .position(LabelPosition::Right)
                            .formatter("{b}"),
                    )
                    .label_layout(LabelLayout::new().hide_overlap(true))
                    .scale_limit(ScaleLimit::new().min(0.4).max(2.0))
                    .line_style(LineStyle::new().color("source").curveness(0.3))
                    .data(data),
            )
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

impl BaseChartTrait for MindGraphChart {
    fn default() -> Self
    where
        Self: Sized,
    {
        MindGraphChart {
            base: super::BaseChart::new().set_type("graph".to_string()),
            data: GraphChartData {
                data: "".to_string(),
            },
            title: "".to_string(),
        }
    }

    fn compose(&self) -> charming::Chart {
        let data: GraphData = serde_json::from_str(&self.data.data).unwrap_or(GraphData {
            nodes: vec![],
            links: vec![],
            categories: vec![],
        });
        charming::Chart::new()
            .legend(Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()))
            .series(
                Graph::new()
                    .name(self.title.clone())
                    .layout(GraphLayout::None)
                    .roam(true)
                    .label(
                        Label::new()
                            .show(true)
                            .position(LabelPosition::Right)
                            .formatter("{b}"),
                    )
                    .label_layout(LabelLayout::new().hide_overlap(true))
                    .scale_limit(ScaleLimit::new().min(0.4).max(2.0))
                    .line_style(LineStyle::new().color("source").curveness(0))
                    .data(data),
            )
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }
}
