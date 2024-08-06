use ratatui::{backend::CrosstermBackend, Terminal};
use crate::ui::UserInterface;
use std::error::Error;
use std::net::TcpStream;
use std::path::PathBuf;
use crossterm::{
    cursor,
    terminal,
    execute,
    ExecutableCommand
};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use nlo_text_editor_server::{ServerAction, ServerResponse, MESSAGE_SIZE};
use std::io::{Read, Write};



// users preferred cursor style
    // Options:
        // DefaultUserShape
        // BlinkingBLock    //inform crossterm of capital L in 'Block'
        // SteadyBlock
        // BlinkingUnderScore
        // SteadyUnderScore
        // BlinkingBar
        // SteadyBar
const CURSOR_STYLE: cursor::SetCursorStyle = cursor::SetCursorStyle::SteadyBlock;
const VIEW_SCROLL_AMOUNT: usize = 1;



#[derive(Clone, Copy)]
pub enum Mode{
    Insert,
    Warning(WarningKind),
    Command,
    FindReplace,
    Goto,
    //Utility(UtilityKind),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarningKind{
    //OpenFileIsModified,
    FocusedFileIsModified,
    FileSaveFailed,
    //FileOpenFailed,
}

pub enum ClientAction{
    Backspace,
    CommandModeAccept,
    CommandModeBackspace,
    CommandModeDelete,
    CommandModeExit,
    CommandModeInsertChar(char),
    CommandModeMoveCursorLeft,
    CommandModeMoveCursorLineEnd,
    CommandModeMoveCursorLineStart,
    CommandModeMoveCursorRight,
    Delete,
    DisplayLineNumbers,
    DisplayStatusBar,
    FindReplaceModeAccept,
    FindReplaceModeBackspace,
    FindReplaceModeDelete,
    FindReplaceModeExit,
    FindReplaceModeInsertChar(char),
    FindReplaceModeMoveCursorLeft,
    FindReplaceModeMoveCursorLineEnd,
    FindReplaceModeMoveCursorLineStart,
    FindReplaceModeMoveCursorRight,
    FindReplaceModeNextInstance,
    FindReplaceModePreviousInstance,
    FindReplaceModeSwitchUtilBarFocus,
    GotoModeAccept,
    GotoModeBackspace,
    GotoModeDelete,
    GotoModeExit,
    GotoModeInsertChar(char),
    GotoModeMoveCursorLeft,
    GotoModeMoveCursorLineEnd,
    GotoModeMoveCursorLineStart,
    GotoModeMoveCursorRight,
    InsertChar(char),
    InsertNewline,
    InsertTab,
    MoveCursorDocumentEnd,
    MoveCursorDocumentStart,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorLineEnd,
    MoveCursorLineStart,
    MoveCursorPageDown,
    MoveCursorPageUp,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorWordEnd,
    MoveCursorWordStart,
    NoOp,
    Quit,
    QuitIgnoringChanges,
    Resize(u16, u16),
    Save,
    ScrollViewDown(usize),
    ScrollViewLeft(usize),
    ScrollViewRight(usize),
    ScrollViewUp(usize),
    SetModeCommand,
    SetModeFindReplace,
    SetModeGoto,
    WarningModeExit,
}



pub struct Application{
    should_quit: bool,
    mode: Mode,
    host_terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    supports_keyboard_enhancement: bool,
    stream: TcpStream,
    ui: UserInterface,
}
impl Application{
    pub fn new() -> Result<Self, Box<dyn Error>>{
        let (mut terminal, supports_keyboard_enhancement) = setup_terminal()?;
        let terminal_size = terminal.size()?;

        let stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => {stream}
            //TODO: if can't connect, spawn new nlo_text_editor_server and retry
            Err(e) => {
                restore_terminal(&mut terminal, supports_keyboard_enhancement)?;
                println!("Could not connect to tcp stream. Make sure you have started an nlo_server process. error: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(Self{
            should_quit: false,
            mode: Mode::Insert,
            host_terminal: terminal,
            supports_keyboard_enhancement,
            stream,
            ui: UserInterface::new(terminal_size)
        })
    }

    //Init: initializes and sets up environment(including loading plugins and setting options?)
    //Event Handling: listens for user input and system events(events can include keystrokes, mouse actions, and signals from the operating system)
        //When an event occurs (e.g., a key is pressed), trigger the corresponding handler function or callback
    //Handler Execution: executes and performs the necessary actions based on the event
        //(For example, if a user presses "Ctrl+S", the corresponding handler might save the current buffer to disk)
    //Redraw and Display: Updates display (or redraws the screen) to reflect any changes resulting from the event handling
    pub fn run(&mut self, file_path: String) -> Result<(), Box<dyn Error>>{
        let path = PathBuf::from(file_path).canonicalize().expect("could not expand relative file path");
        //OPEN FILE
        let response = self.do_ipc_things(ServerAction::OpenFile{file_path: path})?;
        self.process_server_response(response);
        
        self.ui.update_layouts(self.mode);
        //UPDATE CLIENT VIEW SIZE
        let response = self.do_ipc_things(
            ServerAction::UpdateClientViewSize{
                width: self.ui.document_rect().width, height: self.ui.document_rect().height
            }
        )?;
        self.process_server_response(response);

        loop{
            self.ui.update_layouts(self.mode);
            self.ui.render(&mut self.host_terminal, self.mode)?;
            let action = self.handle_event()?;
            self.perform_client_action(action)?;
            if self.should_quit(){
                return Ok(());
            }
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

    pub fn handle_event(&mut self) -> Result<ClientAction, Box<dyn Error>>{
        match event::read()?{
            event::Event::Key(key_event) => {
                Ok(match (key_event, self.mode()){
                    // Insert Mode
                    //(KeyEvent{modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT, code, ..}, Mode::Insert) => {Action::}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorWordEnd}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorWordStart}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentStart}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentEnd}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'),     ..}, Mode::Insert) => {ClientAction::Quit}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('s'),     ..}, Mode::Insert) => {ClientAction::Save}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('g'),     ..}, Mode::Insert) => {ClientAction::SetModeGoto}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('f'),     ..}, Mode::Insert) => {ClientAction::SetModeFindReplace}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('l'),     ..}, Mode::Insert) => {ClientAction::DisplayLineNumbers}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('k'),     ..}, Mode::Insert) => {ClientAction::DisplayStatusBar}
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('o'),     ..}, Mode::Insert) => {ClientAction::SetModeCommand}
                    //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('t'),     ..}, Mode::Insert) => {ClientAction::OpenNewTerminalWindow}
                    (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Char(c), ..}, Mode::Insert) => {ClientAction::InsertChar(c)}
                    (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::ScrollViewDown(VIEW_SCROLL_AMOUNT)}
                    (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::ScrollViewLeft(VIEW_SCROLL_AMOUNT)}
                    (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::ScrollViewRight(VIEW_SCROLL_AMOUNT)}
                    (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::ScrollViewUp(VIEW_SCROLL_AMOUNT)}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Tab,           ..}, Mode::Insert) => {ClientAction::InsertTab}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Enter,         ..}, Mode::Insert) => {ClientAction::InsertNewline}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Delete,        ..}, Mode::Insert) => {ClientAction::Delete}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Backspace,     ..}, Mode::Insert) => {ClientAction::Backspace}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::MoveCursorUp}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::MoveCursorDown}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorLeft}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorRight}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageUp,        ..}, Mode::Insert) => {ClientAction::MoveCursorPageUp}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageDown,      ..}, Mode::Insert) => {ClientAction::MoveCursorPageDown}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorLineStart}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorLineEnd}
                    //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,           ..}, Mode::Insert) => {ClientAction::CollapseSelectionCursor}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Char(c), ..}, Mode::Insert) => {ClientAction::InsertChar(c)}
    
                    // Warning Mode
                    (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'), ..}, Mode::Warning(_)) => {ClientAction::QuitIgnoringChanges}
                    (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,       ..}, Mode::Warning(_)) => {ClientAction::WarningModeExit}
    
                    // Goto Mode
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Esc,           ..}, Mode::Goto) => {ClientAction::GotoModeExit}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Enter,         ..}, Mode::Goto) => {ClientAction::GotoModeAccept}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Backspace,     ..}, Mode::Goto) => {ClientAction::GotoModeBackspace}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Delete,        ..}, Mode::Goto) => {ClientAction::GotoModeDelete}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Right,         ..}, Mode::Goto) => {ClientAction::GotoModeMoveCursorRight}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Left,          ..}, Mode::Goto) => {ClientAction::GotoModeMoveCursorLeft}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Home,          ..}, Mode::Goto) => {ClientAction::GotoModeMoveCursorLineStart}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::End,           ..}, Mode::Goto) => {ClientAction::GotoModeMoveCursorLineEnd}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(c), ..}, Mode::Goto) => {ClientAction::GotoModeInsertChar(c)}
                
                    // FindReplace Mode
                    (KeyEvent{modifiers: KeyModifiers::SHIFT, code: KeyCode::Char(c), ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeInsertChar(c)}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Esc,           ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeExit}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Tab,           ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeSwitchUtilBarFocus}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Up,            ..}, Mode::FindReplace) => {ClientAction::FindReplaceModePreviousInstance}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Down,          ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeNextInstance}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Backspace,     ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeBackspace}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Delete,        ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeDelete}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Right,         ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeMoveCursorRight}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Left,          ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeMoveCursorLeft}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Home,          ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeMoveCursorLineStart}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::End,           ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeMoveCursorLineEnd}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Enter,         ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeAccept}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(c), ..}, Mode::FindReplace) => {ClientAction::FindReplaceModeInsertChar(c)}
                
                    // Command Mode
                    (KeyEvent{modifiers: KeyModifiers::SHIFT, code: KeyCode::Char(c), ..}, Mode::Command) => {ClientAction::CommandModeInsertChar(c)}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Esc,           ..}, Mode::Command) => {ClientAction::CommandModeExit}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(c), ..}, Mode::Command) => {ClientAction::CommandModeInsertChar(c)}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Enter,         ..}, Mode::Command) => {ClientAction::CommandModeAccept}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Backspace,     ..}, Mode::Command) => {ClientAction::CommandModeBackspace}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Delete,        ..}, Mode::Command) => {ClientAction::CommandModeDelete}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Right,         ..}, Mode::Command) => {ClientAction::CommandModeMoveCursorRight}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Left,          ..}, Mode::Command) => {ClientAction::CommandModeMoveCursorLeft}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Home,          ..}, Mode::Command) => {ClientAction::CommandModeMoveCursorLineStart}
                    (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::End,           ..}, Mode::Command) => {ClientAction::CommandModeMoveCursorLineEnd}
    
                    // unhandled keybinds
                    _ => {ClientAction::NoOp}
                })
            },
            event::Event::Resize(x, y) => {Ok(ClientAction::Resize(x, y))}
            _ => {Ok(ClientAction::NoOp)}
        }
    }

    pub fn perform_client_action(&mut self, action: ClientAction) -> Result<(), Box<dyn Error>>{
        match action{
            ClientAction::Backspace => {
                let response = self.do_ipc_things(ServerAction::Backspace)?;
                self.process_server_response(response);
            }
            ClientAction::CommandModeAccept => {
                //if parse_command(editor, ui.util_bar().text()).is_ok(){
                    self.ui.util_bar_mut().clear();
                    self.ui.util_bar_mut().set_offset(0);
                    self.set_mode(Mode::Insert);
                //}
                //ui.scroll(editor);
    
                //TODO: send action request to server
            }
            ClientAction::CommandModeBackspace => {
                self.ui.util_bar_mut().backspace();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeDelete => {
                self.ui.util_bar_mut().delete();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeExit => {
                self.ui.util_bar_mut().clear();
                self.ui.util_bar_mut().set_offset(0);
                self.set_mode(Mode::Insert);
            }
            ClientAction::CommandModeInsertChar(c) => {
                self.ui.util_bar_mut().insert_char(c);
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeMoveCursorLeft => {
                self.ui.util_bar_mut().move_cursor_left();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeMoveCursorLineEnd => {
                self.ui.util_bar_mut().move_cursor_end();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeMoveCursorLineStart => {
                self.ui.util_bar_mut().move_cursor_home();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::CommandModeMoveCursorRight => {
                self.ui.util_bar_mut().move_cursor_right();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::Delete => {
                let response = self.do_ipc_things(ServerAction::Delete)?;
                self.process_server_response(response);
            }
            ClientAction::DisplayLineNumbers => {
                self.ui.set_display_line_numbers(!self.ui.display_line_numbers());
                
                self.ui.update_layouts(self.mode);
                let response = self.do_ipc_things(
                    ServerAction::UpdateClientViewSize{
                        width: self.ui.document_rect().width, 
                        height: self.ui.document_rect().height
                    }
                )?;
                self.process_server_response(response);
            }
            ClientAction::DisplayStatusBar => {
                self.ui.set_display_status_bar(!self.ui.display_status_bar());
                
                self.ui.update_layouts(self.mode);
                let response = self.do_ipc_things(
                    ServerAction::UpdateClientViewSize{
                        width: self.ui.document_rect().width, 
                        height: self.ui.document_rect().height
                    }
                )?;
                self.process_server_response(response);
            }
            ClientAction::FindReplaceModeAccept => {}
            ClientAction::FindReplaceModeBackspace => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().backspace();
                }else{
                    self.ui.util_bar_mut().backspace();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
    
                //run text validity check
                //if let Some(doc) = editor.document(){
                //    if !doc.lines_as_single_string().contains(&ui.util_bar().text()){
                //        ui.util_bar_mut().set_text_is_valid(false);
                //    }else{
                //        ui.util_bar_mut().set_text_is_valid(true);
                //    }
                //}
            }
            ClientAction::FindReplaceModeDelete => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().delete();
                }else{
                    self.ui.util_bar_mut().delete();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
    
                //run text validity check
                //if let Some(doc) = editor.document(){
                //    if !doc.lines_as_single_string().contains(&ui.util_bar().text()){
                //        ui.util_bar_mut().set_text_is_valid(false);
                //    }else{
                //        ui.util_bar_mut().set_text_is_valid(true);
                //    }
                //}
            }
            ClientAction::FindReplaceModeExit => {
                self.ui.util_bar_mut().clear();
                self.ui.util_bar_alternate_mut().clear();
                self.ui.util_bar_mut().set_offset(0);
                self.ui.util_bar_alternate_mut().set_offset(0);
                self.ui.set_util_bar_alternate_focused(false);
                self.set_mode(Mode::Insert);
            }
            ClientAction::FindReplaceModeInsertChar(c) => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().insert_char(c);
                }else{
                    self.ui.util_bar_mut().insert_char(c);
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
    
                //run text validity check
                //if let Some(doc) = editor.document(){
                //    if !doc.lines_as_single_string().contains(&ui.util_bar().text()){
                //        ui.util_bar_mut().set_text_is_valid(false);
                //    }else{
                //        ui.util_bar_mut().set_text_is_valid(true);
                //    }
                //}
            }
            ClientAction::FindReplaceModeNextInstance => {}
            ClientAction::FindReplaceModePreviousInstance => {}
            ClientAction::FindReplaceModeMoveCursorLeft => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().move_cursor_left();
                }else{
                    self.ui.util_bar_mut().move_cursor_left();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
            }
            ClientAction::FindReplaceModeMoveCursorRight => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().move_cursor_right();
                }else{
                    self.ui.util_bar_mut().move_cursor_right();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
            }
            ClientAction::FindReplaceModeMoveCursorLineEnd => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().move_cursor_end();
                }else{
                    self.ui.util_bar_mut().move_cursor_end();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
            }
            ClientAction::FindReplaceModeMoveCursorLineStart => {
                if self.ui.util_bar_alternate_focused(){
                    self.ui.util_bar_alternate_mut().move_cursor_home();
                }else{
                    self.ui.util_bar_mut().move_cursor_home();
                }
    
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
            }
            ClientAction::FindReplaceModeSwitchUtilBarFocus => {
                self.ui.set_util_bar_alternate_focused(!self.ui.util_bar_alternate_focused());
            }
            ClientAction::GotoModeAccept => {
                if let Ok(line_number) = self.ui.util_bar().text().parse::<usize>(){
                        if line_number.saturating_sub(1) < self.ui.document_length(){
                            let response = self.do_ipc_things(
                                ServerAction::GoTo{line_number: line_number.saturating_sub(1)}
                            )?;
                            self.process_server_response(response);
                            
                            self.ui.util_bar_mut().clear();
                            self.ui.util_bar_mut().set_offset(0);
                            self.set_mode(Mode::Insert);
                        }
                }
            }
            ClientAction::GotoModeBackspace => {
                self.ui.util_bar_mut().backspace();
                self.ui.util_bar_mut().scroll();
    
                // run text validity check
                let mut is_numeric = true;
                for grapheme in self.ui.util_bar().text().chars(){ // .graphemes(true)?
                    if !grapheme.is_ascii_digit(){
                        is_numeric = false;
                    }
                }
                let exceeds_doc_length = match self.ui.util_bar().text().parse::<usize>(){
                    Ok(line_number) => {
                        line_number > self.ui.document_length()
                    }
                    Err(_) => false
                };
                if !is_numeric || exceeds_doc_length{
                    self.ui.util_bar_mut().set_text_is_valid(false);
                }else{
                    self.ui.util_bar_mut().set_text_is_valid(true);
                }
            }
            ClientAction::GotoModeDelete => {
                self.ui.util_bar_mut().delete();
                self.ui.util_bar_mut().scroll();
    
                // run text validity check
                let mut is_numeric = true;
                for grapheme in self.ui.util_bar().text().chars(){ // .graphemes(true)?
                    if !grapheme.is_ascii_digit(){
                        is_numeric = false;
                    }
                }
                let exceeds_doc_length = match self.ui.util_bar().text().parse::<usize>(){
                    Ok(line_number) => {
                        line_number > self.ui.document_length()
                    }
                    Err(_) => false
                };
                if !is_numeric || exceeds_doc_length{
                    self.ui.util_bar_mut().set_text_is_valid(false);
                }else{
                    self.ui.util_bar_mut().set_text_is_valid(true);
                }
            }
            ClientAction::GotoModeExit => {
                self.ui.util_bar_mut().clear();
                self.ui.util_bar_mut().set_offset(0);
                self.set_mode(Mode::Insert);
            }
            ClientAction::GotoModeInsertChar(c) => {
                self.ui.util_bar_mut().insert_char(c);
                self.ui.util_bar_mut().scroll();
    
                // run text validity check
                let mut is_numeric = true;
                for grapheme in self.ui.util_bar().text().chars(){ // .graphemes(true)?
                    if !grapheme.is_ascii_digit(){
                        is_numeric = false;
                    }
                }
                let exceeds_doc_length = match self.ui.util_bar().text().parse::<usize>(){
                    Ok(line_number) => {
                        line_number > self.ui.document_length()
                    }
                    Err(_) => false
                };
                if !is_numeric || exceeds_doc_length{
                    self.ui.util_bar_mut().set_text_is_valid(false);
                }else{
                    self.ui.util_bar_mut().set_text_is_valid(true);
                }
            }
            ClientAction::GotoModeMoveCursorLeft => {
                self.ui.util_bar_mut().move_cursor_left();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::GotoModeMoveCursorLineEnd => {
                self.ui.util_bar_mut().move_cursor_end();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::GotoModeMoveCursorLineStart => {
                self.ui.util_bar_mut().move_cursor_home();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::GotoModeMoveCursorRight => {
                self.ui.util_bar_mut().move_cursor_right();
                self.ui.util_bar_mut().scroll();
            }
            ClientAction::InsertChar(c) => {
                let response = self.do_ipc_things(ServerAction::InserChar(c))?;
                self.process_server_response(response);
            }
            ClientAction::InsertNewline => {
                let response = self.do_ipc_things(ServerAction::InsertNewline)?;
                self.process_server_response(response);
            }
            ClientAction::InsertTab => {
                let response = self.do_ipc_things(ServerAction::InsertTab)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorDocumentEnd => {
                let response = self.do_ipc_things(ServerAction::MoveCursorDocumentEnd)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorDocumentStart => {
                let response = self.do_ipc_things(ServerAction::MoveCursorDocumentStart)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorDown => {
                let response = self.do_ipc_things(ServerAction::MoveCursorDown)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorLeft => {
                let response = self.do_ipc_things(ServerAction::MoveCursorLeft)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorLineEnd => {
                let response = self.do_ipc_things(ServerAction::MoveCursorLineEnd)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorLineStart => {
                let response = self.do_ipc_things(ServerAction::MoveCursorLineStart)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorRight => {
                let response = self.do_ipc_things(ServerAction::MoveCursorRight)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorPageUp => {
                let response = self.do_ipc_things(ServerAction::MoveCursorPageUp)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorPageDown => {
                let response = self.do_ipc_things(ServerAction::MoveCursorPageDown)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorUp => {
                let response = self.do_ipc_things(ServerAction::MoveCursorUp)?;
                self.process_server_response(response);
            }
            ClientAction::MoveCursorWordStart => {}
            ClientAction::MoveCursorWordEnd => {}
            ClientAction::NoOp => {}
            ClientAction::Quit => {
                if self.ui.document_modified(){
                    self.set_mode(Mode::Warning(WarningKind::FocusedFileIsModified));
                }else{
                    self.set_should_quit(true);

                    // send server a close action
                    let server_action = ServerAction::CloseConnection;
                    let serialized_server_action = ron::to_string(&server_action)?;
                    match self.stream.write(serialized_server_action.as_bytes()){
                        Ok(_bytes_written) => {
                            //if bytes_written == 0{} else {}
                        }
                        Err(e) => {return Err(Box::new(e));}
                    }
                    self.stream.flush()?;
                }
            }
            ClientAction::QuitIgnoringChanges => {
                self.set_should_quit(true);

                // send server a close action
                let server_action = ServerAction::CloseConnection;
                let serialized_server_action = ron::to_string(&server_action)?;
                match self.stream.write(serialized_server_action.as_bytes()){
                    Ok(_bytes_written) => {
                        //if bytes_written == 0{} else {}
                    }
                    Err(e) => {return Err(Box::new(e));}
                }
                self.stream.flush()?;
            }
            ClientAction::Resize(x, y) => {
                self.ui.set_terminal_size(x, y);
                self.ui.update_layouts(self.mode);
                self.ui.util_bar_mut().scroll();
                self.ui.util_bar_alternate_mut().scroll();
                let response = self.do_ipc_things(
                    ServerAction::UpdateClientViewSize{
                        width: self.ui.document_rect().width, 
                        height: self.ui.document_rect().height
                    }
                )?;
                self.process_server_response(response);
            }
            ClientAction::Save => {
                let response = self.do_ipc_things(ServerAction::Save)?;
                self.process_server_response(response);
            }
            ClientAction::ScrollViewDown(amount) => {
                let response = self.do_ipc_things(ServerAction::ScrollClientViewDown{amount})?;
                self.process_server_response(response);
            }
            ClientAction::ScrollViewLeft(amount) => {
                let response = self.do_ipc_things(ServerAction::ScrollClientViewLeft{amount})?;
                self.process_server_response(response);
            }
            ClientAction::ScrollViewRight(amount) => {
                let response = self.do_ipc_things(ServerAction::ScrollClientViewRight{amount})?;
                self.process_server_response(response);
            }
            ClientAction::ScrollViewUp(amount) => {
                let response = self.do_ipc_things(ServerAction::ScrollClientViewUp{amount})?;
                self.process_server_response(response);
            }
            ClientAction::SetModeCommand => {self.set_mode(Mode::Command)}
            ClientAction::SetModeFindReplace => {self.set_mode(Mode::FindReplace)}
            ClientAction::SetModeGoto => {self.set_mode(Mode::Goto)}
            ClientAction::WarningModeExit => {self.set_mode(Mode::Insert)}
        }
    
        Ok(())
    }

    pub fn process_server_response(&mut self, response: ServerResponse){
        match response{
            ServerResponse::FileOpened{file_name, document_length} => {
                self.ui.set_file_name(file_name);
                self.ui.set_document_length(document_length);
            }
            ServerResponse::ConnectionSucceeded => {}
            ServerResponse::Acknowledge => {}
            ServerResponse::DisplayView{content, line_numbers, client_cursor_positions, document_cursor_position, modified} => {
                self.ui.set_text_in_view(content);
                self.ui.set_line_numbers_in_view(line_numbers);
                self.ui.set_client_cursor_position(client_cursor_positions);
                self.ui.set_document_cursor_position(document_cursor_position);
                self.ui.set_document_modified(modified);
            }
            ServerResponse::Failed(_) => {}
            ServerResponse::CursorPosition{client_cursor_positions, document_cursor_position} => {
                self.ui.set_client_cursor_position(client_cursor_positions);
                self.ui.set_document_cursor_position(document_cursor_position);
            }
        }
    }

    pub fn do_ipc_things(&mut self, action: ServerAction) -> Result<ServerResponse, Box<dyn Error>>{
        let serialized_action = ron::to_string(&action)?;
        match self.stream.write(serialized_action.as_bytes()){
            Ok(_bytes_written) => {
                //if bytes_written == 0{} else {}
            }
            Err(e) => {return Err(Box::new(e));}
        }
        self.stream.flush()?;
    
        let mut response_buffer = [0u8; MESSAGE_SIZE];
        match self.stream.read(&mut response_buffer){
            Ok(size) => {
                let my_string = String::from_utf8_lossy(&response_buffer[0..size]);
                match ron::from_str(&my_string){
                    Ok(response) => {Ok(response)},
                    Err(e) => {Err(Box::new(e))}
                }
            }
            Err(e) => {
                println!("An error occurred. {}", e);
                self.stream.shutdown(std::net::Shutdown::Both).unwrap();
                Err(Box::new(e))
            }
        }
    }

    pub fn restore_terminal(&mut self) -> Result<(), Box<dyn Error>>{
        restore_terminal(&mut self.host_terminal, self.supports_keyboard_enhancement)
    }
}

fn setup_terminal() -> Result<(Terminal<CrosstermBackend<std::io::Stdout>>, bool), Box<dyn Error>>{
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    stdout.execute(CURSOR_STYLE)?;
    
    let supports_keyboard_enhancement = terminal::supports_keyboard_enhancement().unwrap_or(false);

    // only allow terminals with enhanced kb protocol support?
    //if !supports_keyboard_enhancement{
    //    panic!("this terminal does not support enhanced keyboard protocols")
    //}
    //
    
    if supports_keyboard_enhancement {
        use event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
        execute!(
            stdout, 
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                //| KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                //| KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                //| KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }

    let terminal = Terminal::new(
        CrosstermBackend::new(stdout)
    )?;

    Ok((terminal, supports_keyboard_enhancement))
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    supports_keyboard_enhancement: bool
) -> Result<(), Box<dyn Error>>{
    if supports_keyboard_enhancement{
        terminal.backend_mut().execute(event::PopKeyboardEnhancementFlags)?;
    }
    terminal::disable_raw_mode()?;
    terminal.backend_mut().execute(crossterm::terminal::LeaveAlternateScreen)?;
    terminal.backend_mut().execute(crossterm::cursor::SetCursorStyle::DefaultUserShape)?;
    terminal.show_cursor()?;
    
    Ok(())
}
