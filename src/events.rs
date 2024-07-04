use crate::application::{AppState, Mode};
use crate::ui::UserInterface;
use crate::do_ipc_things;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use nlo_text_editor_server::{ServerAction, ServerResponse};

const VIEW_SCROLL_AMOUNT: usize = 1;


pub enum ClientAction{
    CommandModeAccept,
    CommandModeBackspace,
    CommandModeDelete,
    CommandModeExit,
    CommandModeInsertChar(char),
    CommandModeMoveCursorLeft,
    CommandModeMoveCursorLineEnd,
    CommandModeMoveCursorLineStart,
    CommandModeMoveCursorRight,
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
    QuitIgnoringChanges,
    Resize(u16, u16),
    ScrollViewDown(usize),
    ScrollViewLeft(usize),
    ScrollViewRight(usize),
    ScrollViewUp(usize),
    SetModeCommand,
    SetModeFindReplace,
    SetModeGoto,
}



pub fn handle_event(app: &mut AppState) -> Result<ClientAction, Box<dyn Error>>{
    match event::read()?{
        event::Event::Key(key_event) => {
            Ok(match (key_event, app.mode()){
                // Insert Mode
                //(KeyEvent{modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT, code, ..}, Mode::Insert) => {Action::}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::IncrementFocusedDocument}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::DecrementFocusedDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorWordEnd}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorWordStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentEnd}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('n'),     ..}, Mode::Insert) => {ClientAction::NewDocument}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'),     ..}, Mode::Insert) => {ClientAction::QuitIgnoringChanges}//{Action::Quit}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w'),     ..}, Mode::Insert) => {ClientAction::CloseDocument}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('s'),     ..}, Mode::Insert) => {ClientAction::Save}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('g'),     ..}, Mode::Insert) => {ClientAction::SetModeGoto}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('f'),     ..}, Mode::Insert) => {ClientAction::SetModeFindReplace}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('l'),     ..}, Mode::Insert) => {ClientAction::DisplayLineNumbers}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('k'),     ..}, Mode::Insert) => {ClientAction::DisplayStatusBar}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('o'),     ..}, Mode::Insert) => {ClientAction::SetModeCommand}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('t'),     ..}, Mode::Insert) => {ClientAction::OpenNewTerminalWindow}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::ScrollViewDown(VIEW_SCROLL_AMOUNT)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::ScrollViewLeft(VIEW_SCROLL_AMOUNT)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::ScrollViewRight(VIEW_SCROLL_AMOUNT)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::ScrollViewUp(VIEW_SCROLL_AMOUNT)}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Tab,           ..}, Mode::Insert) => {ClientAction::InsertTab}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Enter,         ..}, Mode::Insert) => {ClientAction::InsertNewline}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Delete,        ..}, Mode::Insert) => {ClientAction::Delete}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Backspace,     ..}, Mode::Insert) => {ClientAction::Backspace}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::MoveCursorUp}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::MoveCursorDown}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorLeft}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorRight}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageUp,        ..}, Mode::Insert) => {ClientAction::MoveCursorPageUp}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageDown,      ..}, Mode::Insert) => {ClientAction::MoveCursorPageDown}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorLineStart}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorLineEnd}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,           ..}, Mode::Insert) => {ClientAction::CollapseSelectionCursor}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Char(c), ..}, Mode::Insert) => {ClientAction::InsertChar(c)}

                // Warning Mode
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'), ..}, Mode::Warning(_)) => {Action::QuitIgnoringChanges}
                //(KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w'), ..}, Mode::Warning(_)) => {Action::CloseDocumentIgnoringChanges}
                //(KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Esc,       ..}, Mode::Warning(_)) => {Action::WarningModeExit}
                
                // SaveAs Mode
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Enter,         ..}, Mode::SaveAs) => {Action::SaveAsModeAccept}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Delete,        ..}, Mode::SaveAs) => {Action::SaveAsModeDelete}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Backspace,     ..}, Mode::SaveAs) => {Action::SaveAsModeBackspace}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Left,          ..}, Mode::SaveAs) => {Action::SaveAsModeMoveCursorLeft}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Right,         ..}, Mode::SaveAs) => {Action::SaveAsModeMoveCursorRight}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Home,          ..}, Mode::SaveAs) => {Action::SaveAsModeMoveCursorLineStart}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::End,           ..}, Mode::SaveAs) => {Action::SaveAsModeMoveCursorLineEnd}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Esc,           ..}, Mode::SaveAs) => {Action::SaveAsModeClear}
                //(KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(c), ..}, Mode::SaveAs) => {Action::SaveAsModeInsertChar(c)}

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
            })
        },
        event::Event::Resize(x, y) => {Ok(ClientAction::Resize(x, y))}
        _ => {Ok(ClientAction::NoOp)}
    }
}

pub fn perform_client_action(app: &mut AppState, ui: &mut UserInterface, stream: &mut TcpStream, action: ClientAction) -> Result<(), Box<dyn Error>>{
    match action{
        ClientAction::CommandModeAccept => {
            //if parse_command(editor, ui.util_bar().text()).is_ok(){
                ui.util_bar_mut().clear();
                ui.util_bar_mut().set_offset(0);
                app.set_mode(Mode::Insert);
            //}
            //ui.scroll(editor);

            //TODO: send action request to server
        }
        ClientAction::CommandModeBackspace => {
            ui.util_bar_mut().backspace();
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeDelete => {
            ui.util_bar_mut().delete();
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeExit => {
            ui.util_bar_mut().clear();
            ui.util_bar_mut().set_offset(0);
            app.set_mode(Mode::Insert);
        }
        ClientAction::CommandModeInsertChar(c) => {
            ui.util_bar_mut().insert_char(c);
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeMoveCursorLeft => {
            ui.util_bar_mut().move_cursor_left();
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeMoveCursorLineEnd => {
            ui.util_bar_mut().move_cursor_end();
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeMoveCursorLineStart => {
            ui.util_bar_mut().move_cursor_home();
            ui.util_bar_mut().scroll();
        }
        ClientAction::CommandModeMoveCursorRight => {
            ui.util_bar_mut().move_cursor_right();
            ui.util_bar_mut().scroll();
        }
        ClientAction::DisplayLineNumbers => {
            ui.set_display_line_numbers(!ui.display_line_numbers());
            
            // send UpdateClientViewSize request to server
            ui.update_layouts(app);
            let response = do_ipc_things(
                stream, 
                ServerAction::UpdateClientViewSize(ui.document_rect().width, ui.document_rect().height)
            )?;
            process_server_response(response, ui);
            //
        }
        ClientAction::DisplayStatusBar => {
            ui.set_display_status_bar(!ui.display_status_bar());
            
            // send UpdateClientViewSize request to server
            ui.update_layouts(app);
            let response = do_ipc_things(
                stream, 
                ServerAction::UpdateClientViewSize(ui.document_rect().width, ui.document_rect().height)
            )?;
            process_server_response(response, ui);
            //
        }
        ClientAction::FindReplaceModeAccept => {}
        ClientAction::FindReplaceModeBackspace => {
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().backspace();
            }else{
                ui.util_bar_mut().backspace();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();

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
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().delete();
            }else{
                ui.util_bar_mut().delete();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();

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
            ui.util_bar_mut().clear();
            ui.util_bar_alternate_mut().clear();
            ui.util_bar_mut().set_offset(0);
            ui.util_bar_alternate_mut().set_offset(0);
            ui.set_util_bar_alternate_focused(false);
            app.set_mode(Mode::Insert);
        }
        ClientAction::FindReplaceModeInsertChar(c) => {
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().insert_char(c);
            }else{
                ui.util_bar_mut().insert_char(c);
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();

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
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().move_cursor_left();
            }else{
                ui.util_bar_mut().move_cursor_left();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();
        }
        ClientAction::FindReplaceModeMoveCursorRight => {
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().move_cursor_right();
            }else{
                ui.util_bar_mut().move_cursor_right();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();
        }
        ClientAction::FindReplaceModeMoveCursorLineEnd => {
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().move_cursor_end();
            }else{
                ui.util_bar_mut().move_cursor_end();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();
        }
        ClientAction::FindReplaceModeMoveCursorLineStart => {
            if ui.util_bar_alternate_focused(){
                ui.util_bar_alternate_mut().move_cursor_home();
            }else{
                ui.util_bar_mut().move_cursor_home();
            }

            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();
        }
        ClientAction::FindReplaceModeSwitchUtilBarFocus => {
            ui.set_util_bar_alternate_focused(!ui.util_bar_alternate_focused());
        }
        ClientAction::GotoModeAccept => {
            if let Ok(line_number) = ui.util_bar().text().parse::<usize>(){
                //if let Some(doc) = editor.document_mut(){
                //    if doc.go_to(line_number.saturating_sub(1)).is_ok(){
                //        ui.util_bar_mut().clear();
                //        ui.util_bar_mut().set_offset(0);
                //        app.set_mode(Mode::Insert);
                //        ui.scroll(editor);
                //    }
                //}
                if ui.document_open(){
                    if line_number.saturating_sub(1) <= ui.document_length(){
                        ui.util_bar_mut().clear();
                        ui.util_bar_mut().set_offset(0);
                        app.set_mode(Mode::Insert);
                    }
                }

                //TODO: send action request to server
            }
        }
        ClientAction::GotoModeBackspace => {
            ui.util_bar_mut().backspace();
            ui.util_bar_mut().scroll();

            // run text validity check
            let mut is_numeric = true;
            for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                if !grapheme.is_ascii_digit(){
                    is_numeric = false;
                }
            }
            let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                Ok(line_number) => {
                    //if let Some(doc) = editor.document(){
                    //    line_number > doc.len()
                    //}else{true}
                    line_number > ui.document_length()
                }
                Err(_) => false
            };
            if !is_numeric || exceeds_doc_length{
                ui.util_bar_mut().set_text_is_valid(false);
            }else{
                ui.util_bar_mut().set_text_is_valid(true);
            }
        }
        ClientAction::GotoModeDelete => {
            ui.util_bar_mut().delete();
            ui.util_bar_mut().scroll();

            // run text validity check
            let mut is_numeric = true;
            for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                if !grapheme.is_ascii_digit(){
                    is_numeric = false;
                }
            }
            let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                Ok(line_number) => {
                    //if let Some(doc) = editor.document(){
                    //    line_number > doc.len()
                    //}else{true}
                    line_number > ui.document_length()
                }
                Err(_) => false
            };
            if !is_numeric || exceeds_doc_length{
                ui.util_bar_mut().set_text_is_valid(false);
            }else{
                ui.util_bar_mut().set_text_is_valid(true);
            }
        }
        ClientAction::GotoModeExit => {
            ui.util_bar_mut().clear();
            ui.util_bar_mut().set_offset(0);
            app.set_mode(Mode::Insert);
        }
        ClientAction::GotoModeInsertChar(c) => {
            ui.util_bar_mut().insert_char(c);
            ui.util_bar_mut().scroll();

            // run text validity check
            let mut is_numeric = true;
            for grapheme in ui.util_bar().text().chars(){ // .graphemes(true)?
                if !grapheme.is_ascii_digit(){
                    is_numeric = false;
                }
            }
            let exceeds_doc_length = match ui.util_bar().text().parse::<usize>(){
                Ok(line_number) => {
                    //if let Some(doc) = editor.document(){
                    //    line_number > doc.len()
                    //}else{true}
                    line_number > ui.document_length()
                }
                Err(_) => false
            };
            if !is_numeric || exceeds_doc_length{
                ui.util_bar_mut().set_text_is_valid(false);
            }else{
                ui.util_bar_mut().set_text_is_valid(true);
            }
        }
        ClientAction::GotoModeMoveCursorLeft => {
            ui.util_bar_mut().move_cursor_left();
            ui.util_bar_mut().scroll();
        }
        ClientAction::GotoModeMoveCursorLineEnd => {
            ui.util_bar_mut().move_cursor_end();
            ui.util_bar_mut().scroll();
        }
        ClientAction::GotoModeMoveCursorLineStart => {
            ui.util_bar_mut().move_cursor_home();
            ui.util_bar_mut().scroll();
        }
        ClientAction::GotoModeMoveCursorRight => {
            ui.util_bar_mut().move_cursor_right();
            ui.util_bar_mut().scroll();
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
            let response = do_ipc_things(stream, ServerAction::MoveCursorDown)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorLeft => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorLeft)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorLineEnd => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorLineEnd)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorLineStart => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorLineStart)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorRight => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorRight)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorPageUp => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorPageUp)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorPageDown => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorPageDown)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorUp => {
            let response = do_ipc_things(stream, ServerAction::MoveCursorUp)?;
            process_server_response(response, ui);
        }
        ClientAction::MoveCursorWordStart => {}
        ClientAction::MoveCursorWordEnd => {}
        ClientAction::NoOp => {}
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
        ClientAction::Resize(x, y) => {
            ui.set_terminal_size(x, y);
            ui.update_layouts(app);
            ui.util_bar_mut().scroll();
            ui.util_bar_alternate_mut().scroll();
            let response = do_ipc_things(stream, ServerAction::UpdateClientViewSize(ui.document_rect().width, ui.document_rect().height))?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewDown(amount) => {
            let response = do_ipc_things(stream, ServerAction::ScrollClientViewDown(amount))?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewLeft(amount) => {
            let response = do_ipc_things(stream, ServerAction::ScrollClientViewLeft(amount))?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewRight(amount) => {
            let response = do_ipc_things(stream, ServerAction::ScrollClientViewRight(amount))?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewUp(amount) => {
            let response = do_ipc_things(stream, ServerAction::ScrollClientViewUp(amount))?;
            process_server_response(response, ui);
        }
        ClientAction::SetModeCommand => {app.set_mode(Mode::Command)}
        ClientAction::SetModeFindReplace => {app.set_mode(Mode::FindReplace)}
        ClientAction::SetModeGoto => {app.set_mode(Mode::Goto)}
    }

    Ok(())
}

pub fn process_server_response(response: ServerResponse, ui: &mut UserInterface){
    match response{
        ServerResponse::FileOpened(maybe_file_name, document_length) => {
            ui.set_file_name(maybe_file_name);
            ui.set_document_length(document_length);
        }
        ServerResponse::ConnectionSucceeded => {}
        ServerResponse::Acknowledge => {}
        ServerResponse::DisplayView(content, client_cursor_position, document_cursor_position) => {
            //println!("Client received: {:#?}", content);
            ui.set_text_in_view(content); //TODO: generate a client action instead of directly performing this
            ui.set_client_cursor_position(client_cursor_position);
            ui.set_document_cursor_position(document_cursor_position);
        }
        ServerResponse::Failed(_) => {}
        ServerResponse::CursorPosition(client_cursor_position, document_cursor_position) => {
            ui.set_client_cursor_position(client_cursor_position);
            ui.set_document_cursor_position(document_cursor_position);
        }
    }
}
