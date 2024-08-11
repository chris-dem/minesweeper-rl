#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum BoardCell {
    #[default]
    Empty,
    Mine,
    Selected
}


#[derive(Debug, Clone)]
pub(crate) struct Board {
    bombs: Vec<BoardCell>,
    width: usize,
    height: usize
}



impl Board {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        let bombs = vec![Default::default(); width * height];
        Self {
            width,
            height,
            bombs
        }
    }
}
