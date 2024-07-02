use crate::application::{AppState, Mode};
use crate::ui::UserInterface;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::process::Command;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use nlo_text_editor_server::{ServerAction, ServerResponse};


enum ClientAction{
    Backspace,
    CloseDocument,
    CloseDocumentIgnoringChanges,
    CollapseSelectionCursor,
    CommandModeAccept,
    CommandModeBackspace,
    CommandModeDelete,
    CommandModeExit,
    CommandModeInsertChar(char),
    CommandModeMoveCursorLeft,
    CommandModeMoveCursorLineEnd,
    CommandModeMoveCursorLineStart,
    CommandModeMoveCursorRight,
    DecrementFocusedDocument,
    Delete,
    DisplayLineNumbers,
    DisplayStatusBar,
    ExtendSelectionDown,
    ExtendSelectionLeft,
    ExtendSelectionLineEnd,
    ExtendSelectionLineStart,
    ExtendSelectionRight,
    ExtendSelectionUp,
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
    IncrementFocusedDocument,
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
    NewDocument,
    NoOp,
    OpenNewTerminalWindow,
    Quit,
    QuitIgnoringChanges,
    Save,
    SaveAsModeAccept,
    SaveAsModeBackspace,
    SaveAsModeClear,
    SaveAsModeDelete,
    SaveAsModeInsertChar(char),
    SaveAsModeMoveCursorLeft,
    SaveAsModeMoveCursorLineEnd,
    SaveAsModeMoveCursorLineStart,
    SaveAsModeMoveCursorRight,
    SetModeCommand,
    SetModeFindReplace,
    SetModeGoto,
    WarningModeExit,
}



//pub fn process_event(app: &mut AppState, editor: &mut Editor, ui: &mut UserInterface) -> Result<(), Box<dyn Error>>{
pub fn process_event(app: &mut AppState, ui: &mut UserInterface, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>{
    match event::read()?{
        event::Event::Key(key_event) => {
            let action = match (key_event, app.mode()){
            // Insert Mode
                //(KeyEvent{modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT, code, ..}, Mode::Insert) => {Action::}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::IncrementFocusedDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::DecrementFocusedDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorWordEnd}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorWordStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentEnd}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('n'),     ..}, Mode::Insert) => {ClientAction::NewDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'),     ..}, Mode::Insert) => {ClientAction::QuitIgnoringChanges}//{Action::Quit}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w'),     ..}, Mode::Insert) => {ClientAction::CloseDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('s'),     ..}, Mode::Insert) => {ClientAction::Save}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('g'),     ..}, Mode::Insert) => {ClientAction::SetModeGoto}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('f'),     ..}, Mode::Insert) => {ClientAction::SetModeFindReplace}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('l'),     ..}, Mode::Insert) => {ClientAction::DisplayLineNumbers}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('k'),     ..}, Mode::Insert) => {ClientAction::DisplayStatusBar}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('o'),     ..}, Mode::Insert) => {ClientAction::SetModeCommand}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('t'),     ..}, Mode::Insert) => {ClientAction::OpenNewTerminalWindow}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::ExtendSelectionUp}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::ExtendSelectionDown}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::ExtendSelectionLeft}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::ExtendSelectionRight}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::ExtendSelectionLineStart}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::ExtendSelectionLineEnd}
                (KeyEvent{modifiers: KeyModifiers::SHIFT,   code: KeyCode::Char(c),       ..}, Mode::Insert) => {ClientAction::InsertChar(c)}
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
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,           ..}, Mode::Insert) => {ClientAction::CollapseSelectionCursor}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Char(c), ..}, Mode::Insert) => {ClientAction::InsertChar(c)}
                
            // Warning Mode
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'), ..}, Mode::Warning(_)) => {ClientAction::QuitIgnoringChanges}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w'), ..}, Mode::Warning(_)) => {ClientAction::CloseDocumentIgnoringChanges}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,       ..}, Mode::Warning(_)) => {ClientAction::WarningModeExit}
                
            // SaveAs Mode
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Enter,         ..}, Mode::SaveAs) => {ClientAction::SaveAsModeAccept}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Delete,        ..}, Mode::SaveAs) => {ClientAction::SaveAsModeDelete}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Backspace,     ..}, Mode::SaveAs) => {ClientAction::SaveAsModeBackspace}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Left,          ..}, Mode::SaveAs) => {ClientAction::SaveAsModeMoveCursorLeft}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Right,         ..}, Mode::SaveAs) => {ClientAction::SaveAsModeMoveCursorRight}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Home,          ..}, Mode::SaveAs) => {ClientAction::SaveAsModeMoveCursorLineStart}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::End,           ..}, Mode::SaveAs) => {ClientAction::SaveAsModeMoveCursorLineEnd}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Esc,           ..}, Mode::SaveAs) => {ClientAction::SaveAsModeClear}
                (KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(c), ..}, Mode::SaveAs) => {ClientAction::SaveAsModeInsertChar(c)}

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
            };


    // perform actions
            match action{
                ClientAction::Backspace => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.backspace();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::CloseDocument => {
                    //if let Some(doc) = editor.document(){
                    //    if doc.is_modified(){
                    //        app.set_mode(Mode::Warning(WarningKind::FocusedFileIsModified));
                    //    }else{
                    //        editor.close_document();
                    //    }
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::CloseDocumentIgnoringChanges => {
                    //editor.close_document();
                    //app.set_mode(Mode::Insert);
                    //ui.scroll(editor);
                }
                ClientAction::CollapseSelectionCursor => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.collapse_selection_cursors();
                    //}
                    //app.set_mode(Mode::Insert);
                    //ui.scroll(editor); // this needed here?
                }
                ClientAction::CommandModeAccept => {
                    //if parse_command(editor, ui.util_bar().text()).is_ok(){
                    //if parse_command(ui.util_bar().text()).is_ok(){
                    //    ui.util_bar_mut().clear();
                    //    ui.util_bar_mut().set_offset(0);
                    //    app.set_mode(Mode::Insert);
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::CommandModeBackspace => {
                    //ui.util_bar_mut().backspace();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeDelete => {
                    //ui.util_bar_mut().delete();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeExit => {
                    //ui.util_bar_mut().clear();
                    //ui.util_bar_mut().set_offset(0);
                    //app.set_mode(Mode::Insert);
                }
                ClientAction::CommandModeInsertChar(_) => {
                    //ui.util_bar_mut().insert_char(c);
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeMoveCursorLeft => {
                    //ui.util_bar_mut().move_cursor_left();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeMoveCursorLineEnd => {
                    //ui.util_bar_mut().move_cursor_end();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeMoveCursorLineStart => {
                    //ui.util_bar_mut().move_cursor_home();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::CommandModeMoveCursorRight => {
                    //ui.util_bar_mut().move_cursor_right();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::DecrementFocusedDocument => {
                    //editor.decrement_focused_document();
                    //ui.scroll(editor);
                }
                ClientAction::Delete => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.delete();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::DisplayLineNumbers => {/*ui.set_display_line_numbers(!ui.display_line_numbers())*/}
                ClientAction::DisplayStatusBar => {/*ui.set_display_status_bar(!ui.display_status_bar())*/}
                ClientAction::ExtendSelectionDown => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_down();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::ExtendSelectionLeft => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_left();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::ExtendSelectionLineEnd => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_end();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::ExtendSelectionLineStart => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_home();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::ExtendSelectionRight => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_right();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::ExtendSelectionUp => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.extend_selection_up();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::FindReplaceModeAccept => {}
                ClientAction::FindReplaceModeBackspace => {
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().backspace();
                    //}else{
                    //    ui.util_bar_mut().backspace();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();

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
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().delete();
                    //}else{
                    //    ui.util_bar_mut().delete();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();

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
                    //ui.util_bar_mut().clear();
                    //ui.util_bar_alternate_mut().clear();
                    //ui.util_bar_mut().set_offset(0);
                    //ui.util_bar_alternate_mut().set_offset(0);
                    //ui.set_util_bar_alternate_focused(false);
                    //app.set_mode(Mode::Insert);
                }
                ClientAction::FindReplaceModeInsertChar(_) => {
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().insert_char(c);
                    //}else{
                    //    ui.util_bar_mut().insert_char(c);
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();

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
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().move_cursor_left();
                    //}else{
                    //    ui.util_bar_mut().move_cursor_left();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();
                }
                ClientAction::FindReplaceModeMoveCursorRight => {
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().move_cursor_right();
                    //}else{
                    //    ui.util_bar_mut().move_cursor_right();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();
                }
                ClientAction::FindReplaceModeMoveCursorLineEnd => {
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().move_cursor_end();
                    //}else{
                    //    ui.util_bar_mut().move_cursor_end();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();
                }
                ClientAction::FindReplaceModeMoveCursorLineStart => {
                    //if ui.util_bar_alternate_focused(){
                    //    ui.util_bar_alternate_mut().move_cursor_home();
                    //}else{
                    //    ui.util_bar_mut().move_cursor_home();
                    //}

                    //ui.util_bar_mut().scroll();
                    //ui.util_bar_alternate_mut().scroll();
                }
                ClientAction::FindReplaceModeSwitchUtilBarFocus => {
                    //ui.set_util_bar_alternate_focused(!ui.util_bar_alternate_focused());
                }
                ClientAction::GotoModeAccept => {
                    //if let Ok(line_number) = ui.util_bar().text().parse::<usize>(){
                    //    if let Some(doc) = editor.document_mut(){
                    //        if doc.go_to(line_number.saturating_sub(1)).is_ok(){
                    //            ui.util_bar_mut().clear();
                    //            ui.util_bar_mut().set_offset(0);
                    //            app.set_mode(Mode::Insert);
                    //            ui.scroll(editor);
                    //        }
                    //    }
                    //}
                }
                ClientAction::GotoModeBackspace => {
                    //ui.util_bar_mut().backspace();
                    //ui.util_bar_mut().scroll();

                    // run text validity check
                    //let mut is_numeric = true;
                    //for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                    //    if !grapheme.is_ascii_digit(){
                    //        is_numeric = false;
                    //    }
                    //}
                    //let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                    //    Ok(line_number) => {
                    //        if let Some(doc) = editor.document(){
                    //            line_number > doc.len()
                    //        }else{true}
                    //    }
                    //    Err(_) => false
                    //};
                    //if !is_numeric || exceeds_doc_length{
                    //    ui.util_bar_mut().set_text_is_valid(false);
                    //}else{
                    //    ui.util_bar_mut().set_text_is_valid(true);
                    //}
                }
                ClientAction::GotoModeDelete => {
                    //ui.util_bar_mut().delete();
                    //ui.util_bar_mut().scroll();

                    // run text validity check
                    //let mut is_numeric = true;
                    //for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                    //    if !grapheme.is_ascii_digit(){
                    //        is_numeric = false;
                    //    }
                    //}
                    //let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                    //    Ok(line_number) => {
                    //        if let Some(doc) = editor.document(){
                    //            line_number > doc.len()
                    //        }else{true}
                    //    }
                    //    Err(_) => false
                    //};
                    //if !is_numeric || exceeds_doc_length{
                    //    ui.util_bar_mut().set_text_is_valid(false);
                    //}else{
                    //    ui.util_bar_mut().set_text_is_valid(true);
                    //}
                }
                ClientAction::GotoModeExit => {
                    //ui.util_bar_mut().clear();
                    //ui.util_bar_mut().set_offset(0);
                    //app.set_mode(Mode::Insert);
                }
                ClientAction::GotoModeInsertChar(_) => {
                    //ui.util_bar_mut().insert_char(c);
                    //ui.util_bar_mut().scroll();

                    // run text validity check
                    //let mut is_numeric = true;
                    //for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                    //    if !grapheme.is_ascii_digit(){
                    //        is_numeric = false;
                    //    }
                    //}
                    //let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                    //    Ok(line_number) => {
                    //        if let Some(doc) = editor.document(){
                    //            line_number > doc.len()
                    //        }else{true}
                    //    }
                    //    Err(_) => false
                    //};
                    //if !is_numeric || exceeds_doc_length{
                    //    ui.util_bar_mut().set_text_is_valid(false);
                    //}else{
                    //    ui.util_bar_mut().set_text_is_valid(true);
                    //}
                }
                ClientAction::GotoModeMoveCursorLeft => {
                    //ui.util_bar_mut().move_cursor_left();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::GotoModeMoveCursorLineEnd => {
                    //ui.util_bar_mut().move_cursor_end();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::GotoModeMoveCursorLineStart => {
                    //ui.util_bar_mut().move_cursor_home();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::GotoModeMoveCursorRight => {
                    //ui.util_bar_mut().move_cursor_right();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::IncrementFocusedDocument => {
                    //editor.increment_focused_document();
                    //ui.scroll(editor);
                }
                ClientAction::InsertChar(_) => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.insert_char(c);
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::InsertNewline => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.enter();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::InsertTab => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.tab();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorDocumentEnd => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_document_end();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorDocumentStart => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_document_start();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorDown => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_down();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorLeft => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_left();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorLineEnd => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_end();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorLineStart => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_home();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorRight => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_right();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorPageUp => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_page_up(ui.document_rect().height as usize);
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorPageDown => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_page_down(ui.document_rect().height as usize);
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorUp => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.move_cursor_up();
                    //}
                    //ui.scroll(editor);
                }
                ClientAction::MoveCursorWordStart => {}
                ClientAction::MoveCursorWordEnd => {}
                ClientAction::NewDocument => {
                    //editor.new_document();
                    //ui.scroll(editor);
                }
                ClientAction::NoOp => {}
                ClientAction::OpenNewTerminalWindow => {
                    //open new terminal window at current working directory
                    Command::new("alacritty")
                        .spawn()
                        .expect("failed to spawn new terminal at current directory");
                }
                ClientAction::Quit => {
                    //let mut modified = false;
                    //for doc in editor.documents(){
                    //    if doc.is_modified(){
                    //        modified = true;
                    //    }
                    //}
                    //if modified{
                    //    app.set_mode(Mode::Warning(WarningKind::OpenFileIsModified));
                    //}else{
                    //    app.set_should_quit(true);
                    //}
                }
                ClientAction::QuitIgnoringChanges => {
                    app.set_should_quit(true);
                    //stream.shutdown(std::net::Shutdown::Both).unwrap();
                    // send server a close action
                    let server_action = ServerAction::CloseConnection;
                    let serialized_server_action = ron::to_string(&server_action)?;
                    match stream.write(serialized_server_action.as_bytes()){
                        Ok(bytes_written) => {
                            if bytes_written == 0{} else {}
                        }
                        Err(e) => {return Err(Box::new(e));}
                    }
                    stream.flush()?;
                }
                ClientAction::Save => {
                    //if let Some(doc) = editor.document_mut(){
                    //    if doc.file_name().is_none(){
                    //        app.set_mode(Mode::SaveAs);
                    //    }
                    //    else{
                    //        doc.save()?;
                    //    }
                    //}
                }
                ClientAction::SaveAsModeAccept => {
                    //if let Some(doc) = editor.document_mut(){
                    //    doc.set_file_name(Some(ui.util_bar().text().to_string()));
                    //    if doc.save().is_err(){
                    //        doc.set_file_name(None);
                    //        app.set_mode(Mode::Warning(WarningKind::FileSaveFailed));
                    //        return Ok(());
                    //    }
                    //    app.set_mode(Mode::Insert);
                    //    return Ok(());
                    //}
                }
                ClientAction::SaveAsModeBackspace => {
                    //ui.util_bar_mut().backspace();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeClear => {
                    //ui.util_bar_mut().clear();
                    //ui.util_bar_mut().set_offset(0);
                    //app.set_mode(Mode::Insert);
                }
                ClientAction::SaveAsModeDelete => {
                    //ui.util_bar_mut().delete();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeInsertChar(_) => {
                    //ui.util_bar_mut().insert_char(c);
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeMoveCursorLeft => {
                    //ui.util_bar_mut().move_cursor_left();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeMoveCursorLineEnd => {
                    //ui.util_bar_mut().move_cursor_end();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeMoveCursorLineStart => {
                    //ui.util_bar_mut().move_cursor_home();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SaveAsModeMoveCursorRight => {
                    //ui.util_bar_mut().move_cursor_right();
                    //ui.util_bar_mut().scroll();
                }
                ClientAction::SetModeCommand => {app.set_mode(Mode::Command)}
                ClientAction::SetModeFindReplace => {app.set_mode(Mode::FindReplace)}
                ClientAction::SetModeGoto => {app.set_mode(Mode::Goto)}
                ClientAction::WarningModeExit => {app.set_mode(Mode::Insert)}
            }
        },
        event::Event::Resize(x, y) => {
            ui.set_terminal_size(x, y);
            ui.update_layouts();
            //ui.scroll(editor);
            //ui.util_bar_mut().scroll();
            //ui.util_bar_alternate_mut().scroll();
        }
        _ => {}
    }
    Ok(())
}

pub fn process_server_response(response: ServerResponse, ui: &mut UserInterface){
    match response{
        ServerResponse::ConnectionSucceeded => {}
        ServerResponse::Acknowledge => {}
        ServerResponse::DisplayView(content) => {
            //println!("Client received: {:#?}", content);
            ui.set_text_in_view(content); //TODO: generate a client action instead of directly performing this
        }
        ServerResponse::Failed(_) => {}
    }
}



//pub fn parse_command(editor: &mut Editor, args: &str) -> Result<(), Box<dyn Error>>{
pub fn parse_command(args: &str) -> Result<(), Box<dyn Error>>{
    let mut args = args.split_whitespace();
    
    let command = args.next().unwrap();
    match command{
        //"open_file" => {
        //    let file = args.next().unwrap();
        //    editor.open_document(file)?;
        //}
        //"close_file" => {
        //    editor.close_document();
        //}
        //"increment_focused_document" => editor.increment_focused_document(),
        //"decrement_focused_document" => editor.decrement_focused_document(),
        //"focus_document_at_index" => {
        //    let index: usize = args.next().unwrap().parse().unwrap();
        //    editor.focus_document_at_index(index);
        //}
        "term" => {
            // open new terminal window at current directory.. TODO: fix this closes child when parent closes
            //command: alacritty --working-directory $PWD
            // does this work with $TERM when $TERM isn't alacritty?
            Command::new("alacritty")
            //Command::new("$TERM") //this causes a panic
                //not needed here, because term spawned here defaults to this directory, but good to know
                //.current_dir("/home/j/Documents/programming/rust/nlo_text_editor/")
                //.output() // output keeps current process from working until child process closes
                .spawn()
                .expect("failed to spawn new terminal at current directory");
        }
        _ => {}
    }

    Ok(())
}



// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
//fn flush_resize_events(first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
//    let mut last_resize = first_resize;
//    while let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
//        if let Ok(event::Event::Resize(x, y)) = event::read() {
//            last_resize = (x, y);
//        }
//    }
//
//    (first_resize, last_resize)
//}



//    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
//        if let Event::Key(key) = event::read().context("event read failed")? {
//            return Ok(KeyCode::Char('q') == key.code);
//        }
//    }



#[test]
fn test_crossterm_bitflags(){
    assert!(3 == (KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits()));
}
