

trait Cell{
    fn get_state(&self)->CellState;
    fn set_state(&mut self,state:CellState);
}

#[derive(Clone, Copy)]
enum CellState {
    X,
    O
}

struct SmallCell(CellState);
impl Cell for SmallCell {
    fn get_state(&self) -> CellState{
        self.0
    }

    fn set_state(&mut self,state:CellState) {
        self.0 = state;
    }
}

trait Grid {
    fn get_cell_state(&self,index:usize)->CellState;
    fn occupy_cell(&mut self,index:usize,state:CellState);
    fn verify_grid(&self) -> Option<CellState>;
}

struct BigGrid([Box<dyn Cell>;9]);

impl Grid for BigGrid {
    fn get_cell_state(&self,index:usize)->CellState {
        self.0[index].get_state()
    }

    fn occupy_cell(&mut self,index:usize,state:CellState) {
        self.0[index].set_state(state)
    }

    fn verify_grid(&self) -> Option<CellState> {
        todo!()
    }
}

struct GridCell{
    cell: Box<dyn Cell>,
    grid: Box<dyn Grid>,
}
impl Cell for GridCell{
    fn get_state(&self)->CellState {
        self.cell.get_state()
    }

    fn set_state(&mut self,state:CellState) {
        self.cell.set_state(state)
    }
}
impl Grid for GridCell {
    fn get_cell_state(&self,index:usize)->CellState {
        self.grid.get_cell_state(index)
    }

    fn occupy_cell(&mut self,index:usize,state:CellState) {
        self.grid.occupy_cell(index, state)
    }

    fn verify_grid(&self) -> Option<CellState> {
        self.grid.verify_grid()
    }
}