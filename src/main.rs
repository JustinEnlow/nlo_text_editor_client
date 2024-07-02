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



use nlo_text_editor_client::application::AppState;
use nlo_text_editor_client::ui::UserInterface;
use nlo_text_editor_client::events;
use nlo_text_editor_server::{ServerAction, ServerResponse};
use nlo_text_editor_server::MESSAGE_SIZE;
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
        //TODO: if can't connect, spawn new nlo_text_editor_server and retry
        Err(e) => {
            restore_terminal(&mut terminal, supports_keyboard_enhancement)?;
            println!("Could not connect to tcp stream. error: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(file) = args.pop(){
        ui.update_layouts(); //ensures we get the proper document rect size
        open_file_if_supplied(&mut stream, file, &mut ui)?;
    }
    
    if let Err(e) = run(&mut terminal, &mut app, &mut ui, &mut stream){
        restore_terminal(&mut terminal, supports_keyboard_enhancement)?;
        println!("Encountered an error while running nlo text editor client. error: {}", e);
        return Err(e);
    }

    restore_terminal(&mut terminal, supports_keyboard_enhancement)?;
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

fn open_file_if_supplied(stream: &mut TcpStream, file: String, ui: &mut UserInterface) -> Result<(), Box<dyn Error>>{
    //OPEN FILE
    let action = ServerAction::OpenFile(file);
    send_action_to_server(stream, action)?;
    let response = read_server_response(stream)?;
    ui.set_document_open(true);
    events::process_server_response(response, ui);

    //UPDATE CLIENT VIEW SIZE
    let action = ServerAction::UpdateClientView(ui.document_rect().width, ui.document_rect().height);
    send_action_to_server(stream, action)?;
    let response = read_server_response(stream)?;
    events::process_server_response(response, ui);

    //REQUEST CLIENT VIEW TEXT
    let action = ServerAction::RequestClientViewText;
    send_action_to_server(stream, action)?;
    let response = read_server_response(stream)?;
    events::process_server_response(response, ui);

    Ok(())
}

fn send_action_to_server(stream: &mut TcpStream, action: ServerAction) -> Result<(), Box<dyn Error>>{
    let serialized_action = ron::to_string(&action)?;
    match stream.write(serialized_action.as_bytes()){
        Ok(bytes_written) => {
            if bytes_written == 0{} else {}
        }
        Err(e) => {return Err(Box::new(e));}
    }
    stream.flush()?;

    Ok(())
}

fn read_server_response(stream: &mut TcpStream) -> Result<ServerResponse, Box<dyn Error>>{
    let mut response_buffer = [0u8; MESSAGE_SIZE];
    match stream.read(&mut response_buffer){
        Ok(size) => {
            let my_string = String::from_utf8_lossy(&response_buffer[0..size]);
            match ron::from_str(&my_string){
                Ok(response) => {return Ok(response)},
                Err(e) => {return Err(Box::new(e));}
            };
        }
        Err(e) => {
            println!("An error occurred. {}", e);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return Err(Box::new(e));
        }
    }
}
