mod data_types;
use data_types::{Camera, Color, Ray, Scene, Screen, Sphere};

use minifb::{Key, Window, WindowOptions};
use nalgebra::Vector3;

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let camera: Camera = Camera {
        origin: Vector3::new(0.0, 0.0, 0.0),
        forward: Vector3::new(0.0, 0.0, 1.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        fov: 90.0,
    };

    let screen: Screen = Screen::create_screen(&camera, WIDTH as u32, HEIGHT as u32);

    let scene = Scene {
        sphere: Sphere {
            origin: Vector3::new(0.0, 0.0, 5.0),
            radius2: 3.0,
            color: Color::red(),
        }
    };

    let mut ray = Ray::initial();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                ray.update(x, y, &camera, &screen);
                let mut color = Color::black();

                let (intersected, _distance) = scene.sphere.intersect(&ray);
                
                if intersected {
                    color += scene.sphere.color;
                }

                let index = x + y * WIDTH;
                buffer[index] = color.to_u32();
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
