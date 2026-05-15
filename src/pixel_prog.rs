
pub trait PixelProgram{
	fn color(&self, state: &PipelineState, x: u32, y: u32) -> u32;
}

pub struct PipelineState{

}

pub struct PixelProgManager{
	programs: Vec<Option<Box<dyn PixelProgram>>>,
	state: PipelineState,
}

impl PixelProgManager {
	pub fn dispatch_program<'a>(&'a self, id: u8) -> Option<&'a Box<dyn PixelProgram>>{
		if id as usize >= self.programs.len(){
			None
		}
		else {
			Some((self.programs[id as usize].as_ref())?)
		}
	}

	pub fn pipeline_state<'a>(&'a self) -> &'a PipelineState{
		&self.state
	}

    pub fn new() -> Self {
        Self { programs: vec![], state: PipelineState {  }}
    }
}
