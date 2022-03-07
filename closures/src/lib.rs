use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<'a, T, U, V>
where
    T: Fn(&'a U) -> &'a V,
    U: Eq,
    U: Hash,
{
    calculation: T,
    values: HashMap<&'a U, &'a V>,
}

impl<'a, T, U, V> Cacher<'a, T, U, V>
where
    T: Fn(&'a U) -> &'a V,
    U: Eq,
    U: Hash,
{
    fn new(calculation: T) -> Cacher<'a, T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a U) -> &V {
        match self.values.get(&arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let input1 = 1;
        let input2 = 2;

        let v1 = c.value(&input1);
        let v2 = c.value(&input2);

        assert_eq!(*v2, 2);
    }

    #[test]
    fn call_with_float_values() {
        let mut c = Cacher::new(|a| a);
        let input1 = "biggus";
        let input2 = "dickus";

        let v1 = c.value(&input1);
        let v2 = c.value(&input2);

        assert_eq!(*v2, "dickus");
    }
}
