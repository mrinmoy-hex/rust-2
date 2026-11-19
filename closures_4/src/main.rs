struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // if the value already exists, return it
            Some(v) => v,

            // otherwise execute the closure
            None => {
                let v = (self.calculation)(arg);

                // store the result for future calls
                self.value = Some(v);

                v
            }
        }
    }
}

fn main() {
    let mut expensive = Cacher::new(|num| {
        println!("Calculating...");
        num * 2
    });

    // First call: calculation happens
    println!("{}", expensive.value(10));

    // Second call: cached result is returned
    println!("{}", expensive.value(10));
}
