use std::{collections::HashMap, error::Error};

use plotters::prelude::*;

pub trait Fetch {
    fn fetch(&self) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>>;
}

fn generate_single(name: &String, values: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::<String, usize>::new();

    for value in values {
        *data.entry(value.clone()).or_insert(0) += 1;
    }

    let mut items = data.iter().collect::<Vec<(&String, &usize)>>();
    items.sort_by_key(|item| item.0);
    let max = *items.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;

    let file = format!("{}.png", name);

    let root = BitMapBackend::new(&file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(name, ("sans-serif", 50.0))
        .build_cartesian_2d((0usize..items.len()).into_segmented(), 0usize..max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Cases")
        .x_label_formatter(&|_| String::new())
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    for (idx, &item) in items.iter().enumerate() {
        let color = Palette99::pick(idx).mix(0.9);
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(color.filled())
                    .data(items.iter().enumerate().filter(|(index, _)| index.eq(&idx)).map(|(index, item)| (index, *item.1)))
            )?
            .label(item.0.clone())
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

pub fn generate(fetch: &impl Fetch) -> Result<(), Box<dyn std::error::Error>> {
    for column in fetch.fetch()? {
        generate_single(&column.0, &column.1)?;
    }

    Ok(())
}