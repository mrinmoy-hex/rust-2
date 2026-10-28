#[derive(Debug)]

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

impl Point<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let lst = vec![1, 2, 5, 6, 10, 4];
    let l = largest(&lst);

    println!("The largest value of {lst:?} is: {l}");

    let lst_2 = vec![1.0, 2.2, 5.7, 6.2, 1.0, 4.1];
    let l_2 = largest(&lst_2);

    println!("{l_2}");

    // ---------------------------------------------

    let pt_a = Point::new(5, 10.1);
    let pt_b = Point::new(5.2, 12);

    // pt_b.calculate_distance();
    let pt_c = pt_a.mixup(pt_b);

    println!("Point C = {pt_c:?}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }

    result
}
