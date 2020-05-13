use crate::shared::types::sensor_data::SensorData;
use crate::shared::types::sensor_quality::SensorQuality;
use chrono::Local;
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
            sensor_data.radon_quality(),
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
        sensor_item_quality(sensor_data.radon_quality()),
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
    }
}

fn sensor_quality_into_color(sensor_quality: &SensorQuality) -> Color {
    match *sensor_quality {
        SensorQuality::Good => Color::Green,
        SensorQuality::Bad => Color::Yellow,
        SensorQuality::Terrible => Color::Red,
    }
}

// use crate::gui::app::Message;
// use crate::runner::types::SensorData;
// use chrono::{DateTime, Local, TimeZone};
// use iced::{Align, Color, Column, Container, Element, Length, Row, Text, VerticalAlignment};
// use std::fmt::{Display, Formatter};
//
// #[derive(PartialEq, Eq)]
// pub enum SensorQuality {
//     Good,
//     Bad,
//     Terrible,
// }
//
// impl SensorQuality {
//     fn text(&self) -> &str {
//         match *self {
//             SensorQuality::Good => "GOOD",
//             SensorQuality::Bad => "BAD",
//             SensorQuality::Terrible => "TERRIBLE",
//         }
//     }
//
//     fn color(&self) -> Color {
//         match *self {
//             SensorQuality::Good => Color::from_rgb(0.0, 0.8, 0.0),
//             SensorQuality::Bad => Color::from_rgb(1.0, 0.6, 0.0),
//             SensorQuality::Terrible => Color::from_rgb(1.0, 0.0, 0.0),
//         }
//     }
// }
//
// const TEMPERATURE_UNIT: &str = "C";
// const RADON_UNIT: &str = "Bq/m3";
// const HUMIDITY_UNIT: &str = "%";
// const CO2_UNIT: &str = "ppm";
// const VOC_UNIT: &str = "ppb";
// const PRESSURE: &str = "mbar";
//
// pub fn sensor_item<'a>(
//     label: &str,
//     value: &str,
//     unit: &str,
//     value_quality: &SensorQuality,
// ) -> Element<'a, Message> {
//     let quality_text = value_quality.text();
//
//     Container::new(
//         Column::new()
//             .align_items(Align::Center)
//             .push(Text::new(label).size(20))
//             .push(
//                 Row::new()
//                     .push(
//                         Text::new(value)
//                             .size(30)
//                             .height(Length::Units(40))
//                             .vertical_alignment(VerticalAlignment::Center),
//                     )
//                     .push(Text::new("").width(Length::Units(6)))
//                     .push(
//                         Text::new(unit)
//                             .size(20)
//                             .height(Length::Units(40))
//                             .vertical_alignment(VerticalAlignment::Center),
//                     ),
//             )
//             .push(
//                 Text::new(quality_text)
//                     .size(20)
//                     .color(value_quality.color()),
//             ),
//     )
//     .width(Length::Units(100))
//     .center_x()
//     .into()
// }
//
// pub fn current_sensor_data_screen<'a>(sensor_data: Option<&SensorData>) -> Element<'a, Message> {
//     if sensor_data.is_none() {
//         return Container::new(
//             Column::new()
//                 .width(Length::Fill)
//                 .align_items(Align::Center)
//                 .push(
//                     Text::new("Air quality Dashboard")
//                         .size(40)
//                         .height(Length::Units(50)),
//                 )
//                 .push(Text::new("No sensor data. Either loading or had an error")),
//         )
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .center_x()
//         .center_y()
//         .into();
//     }
//
//     let sensor_data = sensor_data.unwrap();
//
//     Container::new(
//         Column::new()
//             .width(Length::Fill)
//             .align_items(Align::Center)
//             .push(
//                 Text::new("Air quality Dashboard")
//                     .size(40)
//                     .height(Length::Units(50)),
//             )
//             .push(Text::new("Overall air quality"))
//             .push(
//                 Container::new({
//                     let quality = sensor_data.worst_sensor_quality();
//                     Text::new(quality.text()).size(30).color(quality.color())
//                 })
//                 .padding(10),
//             )
//             .push(
//                 Container::new(
//                     Row::new()
//                         .spacing(20)
//                         .width(Length::Fill)
//                         // .align_items(Align::End)
//                         .push(sensor_item(
//                             "RADON",
//                             sensor_data.radon_short_term_average().to_string().as_str(),
//                             RADON_UNIT.to_string().as_str(),
//                             &sensor_data.radon_quality(),
//                         ))
//                         .push(sensor_item(
//                             "TVOC",
//                             sensor_data.voc().to_string().as_str(),
//                             VOC_UNIT,
//                             &sensor_data.voc_quality(),
//                         ))
//                         .push(sensor_item(
//                             "CO2",
//                             sensor_data.co2().to_string().as_str(),
//                             CO2_UNIT,
//                             &sensor_data.co2_quality(),
//                         )),
//                 )
//                 .padding(20),
//             )
//             .push(
//                 // Container::new(
//                 Row::new()
//                     .spacing(20)
//                     // .width(Length::Fill)
//                     // .align_items(Align::End)
//                     .push(sensor_item(
//                         "HUMIDITY",
//                         sensor_data.humidity_in_percent().to_string().as_str(),
//                         HUMIDITY_UNIT,
//                         &sensor_data.humidity_quality(),
//                     ))
//                     .push(sensor_item(
//                         "TEMP",
//                         sensor_data.temperature_in_celsius().to_string().as_str(),
//                         TEMPERATURE_UNIT,
//                         &sensor_data.temperature_quality(),
//                     ))
//                     .push(sensor_item(
//                         "PRESSURE",
//                         sensor_data.atmospheric_pressure().to_string().as_str(),
//                         PRESSURE,
//                         &sensor_data.atmospheric_pressure_quality(),
//                     )),
//                 // )
//                 // .padding(20)
//                 // .center_x()
//                 // .width(Length::Fill)
//                 // .height(Length::Fill),
//             )
//             .push(
//                 Text::new(format!(
//                     "Last checked at: {}",
//                     sensor_data
//                         .timestamp()
//                         .with_timezone(&Local)
//                         .format("%Y-%m-%d %H:%M")
//                 ))
//                 .height(Length::Units(60))
//                 .vertical_alignment(VerticalAlignment::Bottom),
//             ),
//     )
//     .width(Length::Fill)
//     .height(Length::Fill)
//     .center_x()
//     .center_y()
//     .into()
// }
