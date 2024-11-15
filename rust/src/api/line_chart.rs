use chart_core::charts::line_chart::LineChartData;

pub fn new_line_chart(title: String, data: LineChartData) -> Option<Vec<u8>> {
    chart_core::new_line_chart(title, data)
}
