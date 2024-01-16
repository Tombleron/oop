use std::fmt::Display;
use std::iter;
use std::ops::Not;
use std::time::SystemTime;

use cushy::value::{Dynamic, IntoValue, Validations};
use cushy::widget::MakeWidget;
use cushy::widgets::button::ButtonKind;
use cushy::widgets::input::InputValue;
use cushy::widgets::Label;
use cushy::{Run, WithClone};
use figures::units::Lp;

fn main() -> cushy::Result {
    let validations = Validations::default();

    let profit_label = Dynamic::from("0");
    let profit = profit_label.map_each(move |label| label.parse::<f64>().unwrap_or(0.0));
    let title = "Супер мега финансовый калькулятор".h2().centered();

    let input = "Доход:"
        .align_left()
        .and(
            profit_label
                .clone()
                .into_input()
                .validation(validations.validate(&profit_label, validate_float)),
        )
        .into_rows();

    let for_myself = "На себя:"
        .align_left()
        .and(profit.map_each(|val| (val * 0.1).to_string()))
        .into_rows();

    let expanses = "Общие траты:"
        .align_left()
        .and(profit.map_each(|val| (val * 0.7).to_string()))
        .into_rows();

    let piggybank = "В копилку:"
        .align_left()
        .and(profit.map_each(|val| (val * 0.2).to_string()))
        .into_rows();

    let money_block = for_myself
        .expand_horizontally()
        .and(expanses.expand_horizontally())
        .and(piggybank.expand_horizontally())
        .into_columns()
        .contain()
        .expand_horizontally();

    title.and(input).and(money_block).into_rows().run()
}

fn validate_float(input: &String) -> Result<(), &'static str> {
    if let Ok(_) = input.parse::<f64>() {
        Ok(())
    } else {
        Err("Can't be parsed")
    }
}
