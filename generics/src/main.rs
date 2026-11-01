#[derive(Debug)]
// `T` and `U` are independent type parameters, so each coordinate can have a different type.
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // The constructor accepts one value of each coordinate's type.
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    // `mixup` keeps this point's x type and takes the other point's y type.
    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

// This method is available only when both coordinates are `f64`.
impl Point<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // The same generic function works for integer and floating-point slices.
    let lst = vec![1, 2, 5, 6, 10, 4];
    let l = largest(&lst);

    println!("The largest value of {lst:?} is: {l}");

    let lst_2 = vec![1.0, 2.2, 5.7, 6.2, 1.0, 4.1];
    let l_2 = largest(&lst_2);

    println!("The largest floating-point value is: {l_2}");

    // ---------------------------------------------

    let pt_a = Point::new(5, 10.1);
    let pt_b = Point::new(5.2, 12);

    let pt_c = pt_a.mixup(pt_b);
    println!("Point C = {pt_c:?}");

    // A point with two `f64` coordinates has access to the specialized method.
    let pt_d = Point::new(3.0, 4.0);
    println!(
        "Distance of {pt_d:?} from the origin = {}",
        pt_d.calculate_distance()
    );
}

// `PartialOrd` is the trait bound that allows values of `T` to be compared.
// The slice must be non-empty because the first element initializes the current maximum.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }

    result
}
