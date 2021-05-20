struct Point {
    x: f32,
    y: f32,
}

struct Rect {
    top_left: Point,
    bottom_right: Point
}

fn get_rect_area(rect: Rect) -> f32 {
    let Rect {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right: Point {
            x: bottom_right_x,
            y: bottom_right_y,
        }
    } = rect;

    return (top_left_y - bottom_right_y) * (bottom_right_x - top_left_x);
}

fn main() {
    let rect = Rect {
        top_left: Point { x: 1.0, y: 5.0 },
        bottom_right: Point { x: 3.0, y: 2.0 }
    };
    println!("Area of rectangle is {}", get_rect_area(rect));
}


