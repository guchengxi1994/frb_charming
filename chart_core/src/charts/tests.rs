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

    #[test]
    fn mind_graph_test() -> anyhow::Result<()> {
        let graph_data = r#"
        {
  "nodes": [
    {
      "id": "0",
      "name": "Root",
      "symbolSize": 20,
      "x": 0,
      "y": 0,
      "value": 50,
      "category": 0
    },
    {
      "id": "1",
      "name": "Category 1",
      "symbolSize": 15,
      "x": -100,
      "y": 100,
      "value": 30,
      "category": 1
    },
    {
      "id": "2",
      "name": "Category 2",
      "symbolSize": 15,
      "x": 100,
      "y": 100,
      "value": 30,
      "category": 1
    },
    {
      "id": "3",
      "name": "Subcategory 1.1",
      "symbolSize": 10,
      "x": -150,
      "y": 200,
      "value": 20,
      "category": 2
    },
    {
      "id": "4",
      "name": "Subcategory 1.2",
      "symbolSize": 10,
      "x": -50,
      "y": 200,
      "value": 20,
      "category": 2
    },
    {
      "id": "5",
      "name": "Subcategory 2.1",
      "symbolSize": 10,
      "x": 50,
      "y": 200,
      "value": 20,
      "category": 2
    },
    {
      "id": "6",
      "name": "Subcategory 2.2",
      "symbolSize": 10,
      "x": 150,
      "y": 200,
      "value": 20,
      "category": 2
    }
  ],
  "links": [
    { "source": "0", "target": "1" },
    { "source": "0", "target": "2" },
    { "source": "1", "target": "3" },
    { "source": "1", "target": "4" },
    { "source": "2", "target": "5" },
    { "source": "2", "target": "6" }
  ],
  "categories": [
    { "name": "Root" },
    { "name": "Category" },
    { "name": "Subcategory" }
  ]
}
        "#;

        let r = compose(
            "mind_graph".to_owned(),
            "new mind graph chart".to_owned(),
            GraphChartData {
                data: graph_data.to_string(),
            },
            Some(800),
            Some(600),
            None,
        )?;

        File::create("mind_graph.png")?.write_all(&r)?;

        anyhow::Ok(())
    }
}
