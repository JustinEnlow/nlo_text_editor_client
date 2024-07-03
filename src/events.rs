use crate::application::{AppState, Mode};
use crate::ui::UserInterface;
use crate::{send_action_to_server, read_server_response};
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use nlo_text_editor_server::{ServerAction, ServerResponse};


pub enum ClientAction{
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
    ScrollViewDown(usize),
    ScrollViewLeft(usize),
    ScrollViewRight(usize),
    ScrollViewUp(usize),
}



//pub fn process_event(app: &mut AppState, editor: &mut Editor, ui: &mut UserInterface) -> Result<(), Box<dyn Error>>{
pub fn process_event(app: &mut AppState, ui: &mut UserInterface, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>{
    match event::read()?{
        event::Event::Key(key_event) => {
            let action = match (key_event, app.mode()){
                // Insert Mode
                //(KeyEvent{modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT, code, ..}, Mode::Insert) => {Action::}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorWordEnd}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorWordStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentStart}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorDocumentEnd}
                (KeyEvent{modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q'),     ..}, Mode::Insert) => {ClientAction::QuitIgnoringChanges}//{Action::Quit}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::ScrollViewDown(1)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::ScrollViewLeft(1)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::ScrollViewRight(1)}
                (KeyEvent{modifiers: KeyModifiers::ALT,     code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::ScrollViewUp(1)}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Up,            ..}, Mode::Insert) => {ClientAction::MoveCursorUp}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Down,          ..}, Mode::Insert) => {ClientAction::MoveCursorDown}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Left,          ..}, Mode::Insert) => {ClientAction::MoveCursorLeft}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Right,         ..}, Mode::Insert) => {ClientAction::MoveCursorRight}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageUp,        ..}, Mode::Insert) => {ClientAction::MoveCursorPageUp}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::PageDown,      ..}, Mode::Insert) => {ClientAction::MoveCursorPageDown}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::Home,          ..}, Mode::Insert) => {ClientAction::MoveCursorLineStart}
                (KeyEvent{modifiers: KeyModifiers::NONE,    code: KeyCode::End,           ..}, Mode::Insert) => {ClientAction::MoveCursorLineEnd}

                // unhandled keybinds
                _ => {ClientAction::NoOp}
            };

            // perform actions
            perform_client_action(app, ui, stream, action)?;
        },
        event::Event::Resize(x, y) => {
            ui.set_terminal_size(x, y);
            ui.update_layouts();
            //ui.scroll(editor);
            //ui.util_bar_mut().scroll();
            //ui.util_bar_alternate_mut().scroll();

            // let server know of change. this somehow seems to work without calling RequestClientViewText. not sure how. magic?
            let action = ServerAction::UpdateClientViewSize(ui.document_rect().width, ui.document_rect().height);
            send_action_to_server(stream, action)?;
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            
            // send RequestClientViewText to server?
            let action = ServerAction::RequestClientViewText;
            send_action_to_server(stream, action)?;
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            //
        }
        _ => {}
    }
    Ok(())
}

pub fn perform_client_action(app: &mut AppState, ui: &mut UserInterface, stream: &mut TcpStream, action: ClientAction) -> Result<(), Box<dyn Error>>{
    match action{
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
        ClientAction::ScrollViewDown(amount) => {
            // send scroll view action to server
            let action = ServerAction::ScrollClientViewDown(amount);
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            
            // request client view text
            let action = ServerAction::RequestClientViewText;
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);

            //TODO: update client cursor position
        }
        ClientAction::ScrollViewLeft(amount) => {
            // send scroll view action to server
            let action = ServerAction::ScrollClientViewLeft(amount);
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            
            // request client view text
            let action = ServerAction::RequestClientViewText;
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewRight(amount) => {
            // send scroll view action to server
            let action = ServerAction::ScrollClientViewRight(amount);
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            
            // request client view text
            let action = ServerAction::RequestClientViewText;
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
        }
        ClientAction::ScrollViewUp(amount) => {
            // send scroll view action to server
            let action = ServerAction::ScrollClientViewUp(amount);
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
            
            // request client view text
            let action = ServerAction::RequestClientViewText;
            send_action_to_server(stream, action)?;
            // read response
            let response = read_server_response(stream)?;
            process_server_response(response, ui);
        }
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
