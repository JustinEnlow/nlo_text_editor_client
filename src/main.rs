// users preferred cursor style
    // Options:
        // DefaultUserShape
        // BlinkingBLock    //inform crossterm of capital L in 'Block'
        // SteadyBlock
        // BlinkingUnderScore
        // SteadyUnderScore
        // BlinkingBar
        // SteadyBar
const CURSOR_STYLE: cursor::SetCursorStyle = cursor::SetCursorStyle::BlinkingBar;

const MESSAGE_SIZE: usize = 1024;



use nlo_text_editor_client::application::AppState;
use nlo_text_editor_client::ui::UserInterface;
use nlo_text_editor_client::events;
use nlo_text_editor_server::{ServerAction, ServerResponse};
use ratatui::{Terminal, prelude::CrosstermBackend};
use crossterm::{
    cursor,
    terminal,
    event,
    execute,
    ExecutableCommand
};
use std::error::Error;
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>>{
    let (mut terminal, supports_keyboard_enhancement) = setup_terminal()?;
    let mut ui = UserInterface::new(terminal.size()?);
    let mut app = AppState::new();

    let mut stream = match TcpStream::connect("127.0.0.1:7878"){
        Ok(stream) => {stream}
        // if can't connect, spawn new nlo_text_editor_server and retry
        Err(e) => return Err(Box::new(e))
    };

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let arg = args.pop();
    // tell editor to open file, if one supplied
    open_file_if_supplied(&mut stream, arg, &mut ui)?;
    

    let result = run(&mut terminal, &mut app, &mut ui, &mut stream);

    restore_terminal(&mut terminal, supports_keyboard_enhancement)?;

    if result.is_err(){
        println!("{:?}", result);
    }

    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    app: &mut AppState, 
    ui: &mut UserInterface,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn Error>>{
    loop{
        if app.should_quit(){
            return Ok(());
        }
        ui.render(terminal, app)?;
        events::process_event(app, ui, stream)?; // send &mut TcpStream as arg here?
        // read response from server
        // act on response
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

fn restore_terminal(
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

fn open_file_if_supplied(stream: &mut TcpStream, arg: Option<String>, ui: &mut UserInterface) -> Result<(), Box<dyn Error>>{
    if let Some(file) = arg{
        let action = ServerAction::OpenFile(file);
        let serialized_action = ron::to_string(&action)?;
        match stream.write(serialized_action.as_bytes()){
            Ok(bytes_written) => {
                if bytes_written == 0{} else {}
            }
            Err(e) => {return Err(Box::new(e));}
        }
        stream.flush()?;

        // read response from server
        let mut response_buffer = [0u8; MESSAGE_SIZE];
        match stream.read(&mut response_buffer){
            Ok(size) => {
                let my_string = String::from_utf8_lossy(&response_buffer[0..size]);
                let server_response: ServerResponse = match ron::from_str(&my_string){
                    Ok(response) => {response},
                    Err(e) => {return Err(Box::new(e));}
                };
                //println!("Client received: {:#?}", server_response);
                ui.set_document_open(true);
                events::process_server_response(server_response, ui);
            }
            Err(e) => {
                println!("An error occurred. {}", e);
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
        }
    }

    Ok(())
}
