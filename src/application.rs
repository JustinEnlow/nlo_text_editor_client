#[derive(Clone, Copy)]
pub enum Mode{
    Insert,
    Warning(WarningKind),
    SaveAs,
    Command,
    FindReplace,
    Goto,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarningKind{
    OpenFileIsModified,
    FocusedFileIsModified,
    FileSaveFailed,
    FileOpenFailed,
}



pub struct AppState{
    should_quit: bool,
    mode: Mode,
}
impl AppState{
    pub fn new() -> Self{
        Self{
            should_quit: false,
            mode: Mode::Insert
        }
    }
    pub fn mode(&self) -> Mode{
        self.mode
    }
    pub fn set_mode(&mut self, mode: Mode){
        self.mode = mode
    }
    pub fn should_quit(&self) -> bool{
        self.should_quit
    }
    pub fn set_should_quit(&mut self, should_quit: bool){
        self.should_quit = should_quit
    }
}