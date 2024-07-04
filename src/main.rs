use crate::application::Application;
use std::error::Error;

mod application;
mod ui;




fn main() -> Result<(), Box<dyn Error>>{
    let mut app = Application::new()?;
    
    if let Err(e) = app.run(){
        app.restore_terminal()?;
        println!("Encountered an error while running nlo text editor client. error: {}", e);
        return Err(e);
    }

    app.restore_terminal()?;
    Ok(())
}
