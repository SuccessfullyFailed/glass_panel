use glass_panel::{ elements::{ Id, Rectangle, Text, Font, Positioned }, tridimensional::{ lenses::OrthographicLens, model::{ materials::SimpleColorMaterial, Mesh }, Entity, Scene }, GlassPanel };
use std::{ thread::sleep, time::{ Duration, Instant } };


fn main() -> Result<(), Box<dyn std::error::Error>> {

	// Settings.
	let scene_size:[usize; 2] = [800, 600];
	let fps:f32 = 60.0;
	let main_font:Font = Font::new("data/unittesting/Roboto.ttf", 0.1)?;

	// Basic display of 3d scene.
	let cube_entity:Entity = Entity::from_obj("DisplayCube", "data/unittesting/default_cube.obj")?;
	let scene:Scene = Scene::new(scene_size[0], scene_size[1], &OrthographicLens::new(), vec![
		Entity::new("StackOfCubes", Mesh::empty()).with_children(vec![
			cube_entity.displaced(&[0.0; 3]),
			cube_entity.displaced(&[50.0, 0.0, 0.0]),
			cube_entity.displaced(&[95.0, 0.0, 0.0]),
			cube_entity.displaced(&[25.0, 0.0, 40.0]).with_children(vec![
				Entity::new("MaterialDisplayCube", cube_entity.mesh().with_material(&SimpleColorMaterial::new(0xFFFF0000, 0xFF00FF00, 0x00000000))).displaced(&[20.0, 0.0, 20.0]).scaled(&[0.3; 3])
			]),
			Entity::from_obj("ArduinoMotor", "models/S3003.obj")?.displaced(&[46.0, 10.0, 33.0]).rotated(&[-90.0, -20.0, 0.0]).scaled(&[1.3; 3]).with_children(vec![
				Entity::from_obj("MotorHat", "models/TopQuad.obj")?.displaced(&[31.0, 0.0, 46.0]).rotated(&[-90.0, 0.0, 0.0])
			])
		]).displaced(&[0.0, 200.0, 0.0]),
	]);
	
	// Create window.
	let window_source:Rectangle = Rectangle::new(scene_size[0], scene_size[1], 0xFF222222, vec![
		&scene
	]);
	let mut window:GlassPanel = GlassPanel::window("3D renderer display", [0, 0, scene_size[0], scene_size[1]], &window_source);
	window.set_max_fps(fps);

	// Main loop.
	let interval:Duration = Duration::from_millis((1000.0 / fps).floor() as u64);
	let start_time:Instant = Instant::now();
 	while window.exist() {
		let frame_start:Instant = Instant::now();
		window.display();

		// Get the scene.
		if let Some(scene_element) = window.source_mut().child_by_name_mut("tridimensional_scene") {
			if let Ok(scene) = scene_element.as_scene() {

				// Rotate the stack of cubes.
				scene.entities_mut()[0].rotate(&[0.0, 0.0, 0.1]);

				// Rotate the motor hat.
				let motor_hat_rotation:[f32; 3] = [-90.0, ((start_time.elapsed().as_millis() % 8000) as f32 / 8000.0 * 3.14 * 2.0 - 3.15).sin() * 90.0, 0.0];
				scene.entity_by_name_mut("MotorHat").unwrap().set_rotation(&motor_hat_rotation);
			}
		}

		// Await interval.
		let processing_time:Duration = Instant::now() - frame_start;
		if processing_time < interval {
			sleep(interval - processing_time);
		}
	}

	Ok(())
}
