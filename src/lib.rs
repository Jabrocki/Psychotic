
/// Basic definitions regarding entities such as Entity and EntityPtr
pub mod entities;
/// Things related to keyboard and mouse input
pub mod event_handler;
pub mod game;
/// Things related to pixel programs which function similarly to fragment shaders
pub mod pixel_prog;
pub mod player;
pub mod renderer;
pub mod texture_manager;

pub mod world;

/// Utility classes related to images and colors
pub mod image;
/// Level generation
pub mod level;
pub mod pixel;

/// Concrete entity types such as Enemy
pub mod entity;

/// Moveable trait
pub mod movement;

pub mod animation;
pub mod shoot;

pub mod pathfinder;
pub mod renderer_ui;
