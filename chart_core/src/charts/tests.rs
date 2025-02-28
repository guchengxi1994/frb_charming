#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{
        charts::{bar_chart::BarChartData, graph_chart::GraphChartData, line_chart::LineChartData},
        compose,
    };

    #[test]
    fn line_test() -> anyhow::Result<()> {
        let line_data = LineChartData {
            x: vec!["周一", "Tue", "Wed", "Thu", "Fri", "Sat", "周日"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            y: vec![120f32, 200f32, 150f32, 80f32, 70f32, 110f32, 130f32],
        };

        let r = compose(
            "line".to_owned(),
            "new line chart".to_owned(),
            line_data,
            Some(800),
            Some(600),
            None,
        )?;

        File::create("line.png")?.write_all(&r)?;

        anyhow::Ok(())
    }

    #[test]
    fn bar_test() -> anyhow::Result<()> {
        let bar_data = BarChartData {
            x: vec!["周一", "Tue", "Wed", "Thu", "Fri", "Sat", "周日"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            y: vec![120f32, 200f32, 150f32, 80f32, 70f32, 110f32, 130f32],
        };

        let r = compose(
            "bar".to_owned(),
            "new bar chart".to_owned(),
            bar_data,
            Some(800),
            Some(600),
            None,
        )?;

        File::create("bar.png")?.write_all(&r)?;

        anyhow::Ok(())
    }

    #[test]
    fn graph_test() -> anyhow::Result<()> {
        let graph_data = include_str!("../../graph.json");

        let r = compose(
            "graph".to_owned(),
            "new graph chart".to_owned(),
            GraphChartData {
                data: graph_data.to_string(),
            },
            Some(800),
            Some(600),
            None,
        )?;

        File::create("graph.png")?.write_all(&r)?;

        anyhow::Ok(())
    }
}
