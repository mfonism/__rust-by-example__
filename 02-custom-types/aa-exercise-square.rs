#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn make_square(bottom_left: Point, size: f32) -> Rectangle {
    let Point {
        x: bottom_left_x,
        y: bottom_left_y,
    } = bottom_left;
    Rectangle {
        top_left: Point { 
            x: bottom_left_x,
            y: bottom_left_y + size
        },
        bottom_right: Point {
            x: bottom_left_x + size,
            y: bottom_left_y
        }
    }
}

fn main() {
    let bottom_left = Point {
        x: -1f32,
        y: 2f32,
    };
    let size = 2f32;
    println!("{:?}", make_square(bottom_left, size));
}