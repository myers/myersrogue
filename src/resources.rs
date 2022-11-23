use crate::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Deref, DerefMut)]
pub struct MousePoint(pub Point);

#[derive(Resource, Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Theme(pub Box<dyn MapTheme>);

#[derive(Resource, Deref, DerefMut)]
pub struct GameKeyCode(pub VirtualKeyCode);
