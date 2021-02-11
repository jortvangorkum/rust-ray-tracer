use nalgebra::Vector3;

#[macro_export]
macro_rules! profile {
    ($name:tt $execution:expr) => {
        {
            let now = std::time::Instant::now();
            $execution;
            let elapsed = now.elapsed().as_millis();
            println!("{}: {}ms", $name as &str, elapsed);
        }
    }
}