use three::controls::OrbitControls;
use three::window::Window;
use three::Object;

struct Particle {
    x: f32,
    y: f32,
    z: f32,
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
    mass: f32,
}
fn main() {
    let window = Window::new("Star Formation", "/assets");

    let mut camera = window.factory.perspective_camera(60.0, 0.1, 1000.0);
    camera.set_position([0.0, 0.0, 500.0]);

    let mut scene = window.factory.scene();

    let mut star = window.factory.sphere(50.0, 32, 32);
    star.set_color(0xffff00);
    star.set_position([0.0, 0.0, 0.0]);
    scene.add(&star);

    let mut particles = Vec::new();
    for _ in 0..1000 {
        let particle = Particle {
            x: rand::random::<f32>() * 800.0 - 400.0,
            y: rand::random::<f32>() * 800.0 - 400.0,
            z: rand::random::<f32>() * 800.0 - 400.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 0.0,
            mass: rand::random::<f32>() * 4.0 + 1.0,
        };
        let mut mesh = window.factory.sphere(particle.mass * 2.0, 16, 16);
        mesh.set_position([particle.x, particle.y, particle.z]);
        scene.add(&mesh);
        particles.push(particle);
    }
}
