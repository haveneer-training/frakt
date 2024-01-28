use complex::Complex;
use shared::fractal::{FractalDescriptor, Julia};
use shared::image::{Point, Range, Resolution};
use shared::networking::{Task, U8Data};

fn main() {
    // Create a Task
    let task = Task {
        id: U8Data {
            offset: 0,
            count: 0,
        },
        fractal: FractalDescriptor::Julia(Julia {
            c: Complex {
                re: 0.0,
                im: 0.0,
            },
            divergence_threshold_square: 0.0,
        }),
        max_iteration: 0,
        resolution: Resolution {
            nx: 0,
            ny: 0,
        },
        range: Range {
            min: Point {
                x: 0.0,
                y: 0.0,
            },
            max: Point {
                x: 0.0,
                y: 0.0,
            },
        },
    };

    let task = serde_json::to_string(&task).unwrap();
    println!("{task}");

    let task: Task = serde_json::from_str(&task).unwrap();

    println!("{task:?}");
}