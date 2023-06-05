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
}
