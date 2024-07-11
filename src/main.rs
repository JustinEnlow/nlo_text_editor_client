use crate::application::Application;
use std::error::Error;

mod application;
mod ui;




fn main() -> Result<(), Box<dyn Error>>{
    let mut app = Application::new()?;

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(file_path) = args.pop(){
        if let Err(e) = app.run(file_path){
            app.restore_terminal()?;
            println!("Encountered an error while running nlo text editor client. error: {}", e);
            return Err(e);
        }
    }else{
        app.restore_terminal()?;
        println!("Could not open file. No file path provided.");
        return Ok(());
    }

    app.restore_terminal()?;
    Ok(())
}
