mod engine_objects;
use engine_objects::{Camera, Color, Ray, Scene, Screen, lights::PointLight, primitives::Sphere};

use minifb::{Key, Window, WindowOptions};
use nalgebra::{Unit, Vector3};

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut camera: Camera = Camera {
        origin: Vector3::new(0.0, 0.0, 0.0),
        forward: Vector3::new(0.0, 0.0, 1.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        fov: 90.0,
    };

    let mut screen: Screen = Screen::create_screen(&camera, WIDTH as u32, HEIGHT as u32);

    let scene: Scene = Scene {
        lights: vec![
            PointLight {
                origin: Vector3::new(0.0, 2.0, 0.0),
                intensity: 15.0,
            },
        ],
        primitives: vec![
            Box::new(
                Sphere {
                    origin: Vector3::new(0.0, 0.0, 5.0),
                    radius2: 3.0,
                    color: Color::red(),
                }
            ),
            Box::new(
                Sphere {
                    origin: Vector3::new(2.0, 0.0, 4.0),
                    radius2: 3.0,
                    color: Color::blue(),
                }
            ),
            Box::new(
                Sphere {
                    origin: Vector3::new(0.0, 2.0, 4.0),
                    radius2: 3.0,
                    color: Color::green(),
                }
            ),
        ],
    };

    let mut prim_ray = Ray::initial();
    let mut shadow_ray = Ray::initial();

    let mut window = Window::new(
        "Rust Ray Tracer - Jort van Gorkum",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                prim_ray.update_prim(x, y, &camera, &screen);
                let color = prim_ray.trace(&scene, &mut shadow_ray);
                let index = x + y * WIDTH;
                buffer[index] = color.to_u32();
            }
        }

        let elapsed = now.elapsed().as_millis();
        println!("{}ms", elapsed);

        camera.update_input(&window);
        screen.update_screen(&camera);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
