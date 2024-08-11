use std::cell::RefCell;

use crate::board::Board;
use rand::prelude::*;

pub trait Initializer {
    fn initialize(&self, board: &mut Board);
}

pub struct RandomInitializer {
    pub box_rng : RefCell<dyn RngCore>
}

impl RandomInitializer {
    pub fn new(rng : &mut dyn RngCore) -> Self {
        RandomInitializer {
            box_rng: RefCell::new(thread_rng())
        }
    }
}
