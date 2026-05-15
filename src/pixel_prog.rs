use crate::pixel::wall_program::WallProgram;


pub trait PixelProgram{
	fn color(&self, state: &PipelineState, x: u32, y: u32) -> u32;
}

pub struct PipelineState{

}

pub struct PixelProgManager{
	programs: Vec<Option<Box<dyn PixelProgram + Send + Sync>>>,
	state: PipelineState,
}

impl PixelProgManager {
	pub fn dispatch_program<'a>(&'a self, id: u8) -> Option<&'a Box<dyn PixelProgram + Send + Sync>>{
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
    	let mut progs: Vec<Option<Box<dyn PixelProgram + Send + Sync>>> = vec![];

     	progs.push(None);
    	progs.push(Some(Box::new(WallProgram::new()) as Box<dyn PixelProgram + Send + Sync>));

        Self { programs: progs, state: PipelineState {  }}
    }
}
