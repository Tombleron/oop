use std::cmp::Ordering::{Equal, Greater, Less};

trait Operation<T> {
    fn name(&self) -> &'static str;
    fn designation(&self) -> &'static str;
    fn calclate(&self, a: T, b: T) -> T;
}

struct Max;

impl Operation<i8> for Max {
    fn name(&self) -> &'static str {
        "max"
    }

    fn designation(&self) -> &'static str {
        "max"
    }

    fn calclate(&self, a: i8, b: i8) -> i8 {
        a.max(b)
    }
}

struct Min;

impl Operation<i8> for Min {
    fn name(&self) -> &'static str {
        "min"
    }

    fn designation(&self) -> &'static str {
        "min"
    }

    fn calclate(&self, a: i8, b: i8) -> i8 {
        a.min(b)
    }
}

struct AbsDif;

impl Operation<i8> for AbsDif {
    fn name(&self) -> &'static str {
        "absdif"
    }

    fn designation(&self) -> &'static str {
        "absdif"
    }

    fn calclate(&self, a: i8, b: i8) -> i8 {
        (a - b).abs()
    }
}

struct AbsMax;

impl Operation<i8> for AbsMax {
    fn name(&self) -> &'static str {
        "absmax"
    }

    fn designation(&self) -> &'static str {
        "absmax"
    }

    fn calclate(&self, a: i8, b: i8) -> i8 {
        match a.abs().cmp(&b.abs()) {
            Greater => a,
            Less => b,
            Equal => a,
        }
    }
}

struct AbsMin;

impl Operation<i8> for AbsMin {
    fn name(&self) -> &'static str {
        "absmin"
    }

    fn designation(&self) -> &'static str {
        "absmin"
    }

    fn calclate(&self, a: i8, b: i8) -> i8 {
        match a.abs().cmp(&b.abs()) {
            Greater => b,
            Less => a,
            Equal => a,
        }
    }
}

fn main() {
    // Box это ссылка на память в куче. В Rust нельзя создавать объекты, реализующие типаж (trait)
    // на стеке, потому что размер таких объектов неизвестен на этапе компиляции.
    let operations: &[Box<dyn Operation<i8>>] = &[
        Box::new(Max),
        Box::new(Min),
        Box::new(AbsDif),
        Box::new(AbsMax),
        Box::new(AbsMin),
    ];

    let a = 32;
    let b = -17;

    for op in operations {
        println!(
            "{} {} {} = {} ({} {} {} = {}) <-- {}",
            a,
            op.designation(),
            b,
            op.calclate(a, b),
            a,
            op.designation(),
            b,
            op.calclate(a, b),
            op.name(),
        );
    }
}
