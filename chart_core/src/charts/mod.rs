pub mod bar_chart;
pub mod graph_chart;
pub mod line_chart;
mod tests;

use charming::{
    element::{CoordinateSystem, ItemStyle, Label},
    Chart,
};

#[derive(Debug)]
pub struct BaseChart {
    pub type_name: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub coordinate_system: Option<String>,
    pub label: Option<bool>,
    pub item_style: Option<String>,
    pub emphasis: Option<String>,
}

impl BaseChart {
    pub fn new() -> BaseChart {
        BaseChart {
            type_name: None,
            id: None,
            name: None,
            coordinate_system: None,
            label: None,
            item_style: None,
            emphasis: None,
        }
    }

    pub fn set_type(mut self, type_name: String) -> Self {
        self.type_name = Some(type_name);
        return self;
    }

    pub fn get_coordinate_system(&self) -> CoordinateSystem {
        if let Some(coordinate_system) = self.coordinate_system.as_ref() {
            match coordinate_system.as_str() {
                "cartesian2d" => return CoordinateSystem::Cartesian2d,
                "polar" => return CoordinateSystem::Polar,
                "geo" => return CoordinateSystem::Geo,
                "calendar" => return CoordinateSystem::Calendar,
                "single" => return CoordinateSystem::Single,
                "parallel" => return CoordinateSystem::Parallel,
                _ => return CoordinateSystem::Cartesian2d,
            }
        }

        return CoordinateSystem::Cartesian2d;
    }

    pub fn get_label(&self) -> Label {
        if let Some(label) = self.label {
            return Label::new().show(label);
        }

        return Label::new().show(false);
    }

    pub fn get_item_style(&self) -> ItemStyle {
        return ItemStyle::new().border_radius(8);
    }
}

pub trait BaseChartTrait {
    fn default() -> Self
    where
        Self: Sized;

    fn compose(&self) -> Chart;

    fn set_title(&mut self, title: String);
}

pub trait ChartData {
    fn as_chart_data(&self) -> Box<dyn std::any::Any>;
}
