use rand::random;
use three::controls::orbit;
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
    let window = Window::new("Star Formation");
    let mut camera = window.factory.perspective_camera(60.0, 0.1..1000.0);
    camera.set_position([0.0, 0.0, 500.0]);

    let mut scene = window.factory.scene();

    let mut star = window.factory.mesh(
        three::Geometry::uv_sphere(50.0, 32, 16),
        three::material::Basic {
            color: 0xffff00,
            wireframe: false,
        },
    );
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

    let controls = OrbitControls::builder(&window)
        .position([0.0, 0.0, 500.0])
        .target([0.0, 0.0, 0.0])
        .build();

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        for particle in &mut particles {
            let dx = star.position().x - particle.x;
            let dy = star.position().y - particle.y;
            let dz = star.position().z - particle.z;
            let distance_squared = dx * dx + dy * dy + dz * dz;
            let force = 0.1 * star.scale().x / (1.0 + distance_squared);
            particle.speed_x += force * dx;
            particle.speed_y += force * dy;
            particle.speed_z += force * dz;
        }
        for (particle, mesh) in particles.iter_mut().zip(scene.objects_mut()) {
            particle.x += particle.speed_x;
            particle.y += particle.speed_y;
            particle.z += particle.speed_z;
            mesh.set_position([particle.x, particle.y, particle.z]);
        }

        controls.update(&window.input);
        window.render(&camera, &scene);
    }
}
