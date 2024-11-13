#[cfg(test)]
mod tests {
    use charming::{
        component::{Axis, Legend, Title},
        element::{AxisType, ItemStyle},
        series::{Line, Pie, PieRoseType},
        Chart, ImageRenderer,
    };

    #[test]
    fn generate_image() -> anyhow::Result<()> {
        let chart = Chart::new().legend(Legend::new().top("bottom")).series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(vec![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        );

        let mut renderer = ImageRenderer::new(1000, 800);
        renderer.save(&chart, r"D:\github_repo\frb_charming\nightingale.svg")?;
        anyhow::Ok(())
    }

    #[test]
    fn generate_line_image() -> anyhow::Result<()> {
        let chart = Chart::new()
            .title(Title::new().text("Demo: Yew + Charming"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        let mut renderer = ImageRenderer::new(1000, 800);
        renderer.save(&chart, r"D:\github_repo\frb_charming\nightingale_line.svg")?;
        anyhow::Ok(())
    }
}
