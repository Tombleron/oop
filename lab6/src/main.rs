//! This example shows how dynamic values make it easy to communicate state back
//! to widgets from multiple threads.

use std::time::Duration;

use cushy::value::Dynamic;
use cushy::widget::MakeWidget;
use cushy::widgets::progress::Progressable;
use cushy::Run;

fn main() -> cushy::Result {
    let task1 = Dynamic::new(0u8);
    std::thread::spawn({
        let progress = task1.clone();
        move || {
            while progress.get() < 10 {
                std::thread::sleep(Duration::from_millis(400));
                progress.set(progress.get() + 1);
            }
        }
    });
    let task2 = Dynamic::new(0u8);
    std::thread::spawn({
        let progress = task2.clone();
        move || {
            while progress.get() < 10 {
                std::thread::sleep(Duration::from_millis(317));
                progress.set(progress.get() + 1);
            }
        }
    });
    let task3 = Dynamic::new(0u8);
    std::thread::spawn({
        let progress = task3.clone();
        move || {
            while progress.get() < 10 {
                std::thread::sleep(Duration::from_millis(187));
                progress.set(progress.get() + 1);
            }
        }
    });

    task1
        .progress_bar_to(10)
        .contain()
        .expand_horizontally()
        .and(task2.progress_bar_to(10).contain().expand_horizontally())
        .and(task3.progress_bar_to(10).contain().expand_horizontally())
        .into_columns()
        .run()
}
