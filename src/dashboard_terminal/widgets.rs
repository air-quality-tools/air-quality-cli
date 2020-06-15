use crate::shared::types::sensor_data::SensorData;
use crate::shared::types::sensor_quality::SensorQuality;
use chrono::{Local, Utc};
use std::borrow::Cow;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::Frame;

pub fn dashboard_loading<B: Backend>(frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let block = Block::default()
        .title(" Air quality dashboard ")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD))
        .borders(Borders::ALL);

    let text = [Text::raw("Loading sensor data")];

    let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
    frame.render_widget(paragraph, chunks[0]);
}

pub fn dashboard_error<B: Backend>(frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let block = Block::default()
        .title(" Air quality dashboard ")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD))
        .borders(Borders::ALL);

    let text = [Text::raw(
        "Got an error. Could probably not read te sensor data",
    )];

    let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
    frame.render_widget(paragraph, chunks[0]);
}

pub fn dashboard_sensor_data<B: Backend>(frame: &mut Frame<B>, sensor_data: &SensorData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let block = Block::default()
        .title(" Air quality dashboard ")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD))
        .borders(Borders::ALL);

    let overall_quality = sensor_data.worst_sensor_quality();

    let text = [
        Text::raw("Overall air quality: "),
        Text::raw("[ "),
        Text::styled(
            sensor_quality_into_text(&overall_quality),
            Style::default().fg(sensor_quality_into_color(&overall_quality)),
        ),
        Text::raw(" ]"),
        Text::raw("\n"),
        Text::raw(format!(
            "Last checked at: {}",
            sensor_data
                .timestamp()
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M")
        )),
        Text::raw("   "),
        {
            let start = sensor_data.timestamp().with_timezone(&Local);
            let end = Utc::now().with_timezone(&Local);
            let diff = end - start;
            Text::raw(format!("[ {} minutes ago ]", diff.num_minutes()))
        },
    ];

    let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
    frame.render_widget(paragraph, chunks[0]);

    sensor_data_block(frame, chunks[1], sensor_data);
}

fn sensor_data_block<B>(frame: &mut Frame<B>, area: Rect, sensor_data: &SensorData)
where
    B: Backend,
{
    let text = [
        Text::raw("\n"),
        sensor_item_heading("RADON"),
        sensor_item_heading("TVOC"),
        sensor_item_heading("CO2"),
        sensor_item_heading("HUMIDITY"),
        sensor_item_heading("TEMP"),
        sensor_item_heading("PRESSURE"),
        Text::raw("\n"),
        sensor_item_value(
            &sensor_data.radon_short_term_average(),
            "Bq/m3",
            sensor_data.radon_short_term_quality(),
        ),
        sensor_item_value(&sensor_data.voc(), "ppb", sensor_data.voc_quality()),
        sensor_item_value(&sensor_data.co2(), "ppm", sensor_data.co2_quality()),
        sensor_item_value(
            &sensor_data.humidity_in_percent(),
            "%",
            sensor_data.humidity_quality(),
        ),
        sensor_item_value(
            &sensor_data.temperature_in_celsius(),
            "C",
            sensor_data.temperature_quality(),
        ),
        sensor_item_value(
            &sensor_data.atmospheric_pressure(),
            "mbar",
            sensor_data.atmospheric_pressure_quality(),
        ),
        Text::raw("\n"),
        sensor_item_quality(sensor_data.radon_short_term_quality()),
        sensor_item_quality(sensor_data.voc_quality()),
        sensor_item_quality(sensor_data.co2_quality()),
        sensor_item_quality(sensor_data.humidity_quality()),
        sensor_item_quality(sensor_data.temperature_quality()),
        sensor_item_quality(sensor_data.atmospheric_pressure_quality()),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Individual sensor data ");
    let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
    frame.render_widget(paragraph, area);
}

fn sensor_item_heading(heading: &str) -> Text {
    Text::Styled(
        Cow::from(format!("| {: ^10} |", heading)),
        Style::default().modifier(Modifier::BOLD),
    )
}

fn sensor_item_value<'a>(value: &f32, unit: &str, sensor_quality: SensorQuality) -> Text<'a> {
    Text::Styled(
        Cow::from(format!("| {: ^10} |", format!("{} {}", value, unit))),
        Style::default()
            .modifier(Modifier::BOLD)
            .fg(sensor_quality_into_color(&sensor_quality)),
    )
}

fn sensor_item_quality<'a>(sensor_quality: SensorQuality) -> Text<'a> {
    Text::Styled(
        Cow::from(format!(
            "| {: ^10} |",
            sensor_quality_into_text(&sensor_quality).replace("GOOD", "-")
        )),
        Style::default()
            .modifier(Modifier::BOLD)
            .fg(sensor_quality_into_color(&sensor_quality)),
    )
}

fn sensor_quality_into_text(sensor_quality: &SensorQuality) -> &str {
    match *sensor_quality {
        SensorQuality::Good => "GOOD",
        SensorQuality::Bad => "BAD",
        SensorQuality::Terrible => "TERRIBLE",
        SensorQuality::DependsOnContext => "DEPENDS",
    }
}

fn sensor_quality_into_color(sensor_quality: &SensorQuality) -> Color {
    match *sensor_quality {
        SensorQuality::Good => Color::Green,
        SensorQuality::Bad => Color::Yellow,
        SensorQuality::Terrible => Color::Red,
        SensorQuality::DependsOnContext => Color::LightBlue,
    }
}
