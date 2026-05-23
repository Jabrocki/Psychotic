pub struct AnimationState{
	default: u8,

	pub current_state: u8,
	time: f32,
}

impl AnimationState{
	pub fn new(default: u8) -> Self{
		Self{
			default: default,
			current_state: default,
			time: 0.
		}
	}

	pub fn set_frame(&mut self, frame: u8, time: f32){
		self.current_state = frame;
		self.time = time;
	}

	pub fn update(&mut self, delta: f32){
		self.time -= delta;
		if self.time <= 0. && self.current_state != self.default{
			self.current_state = self.default;
		}
	}
}
