use crate::application::Application;
use std::error::Error;

mod application;
mod ui;




fn main() -> Result<(), Box<dyn Error>>{
    let file_path = if std::env::args().count() > 1 && std::env::args().count() < 3{
        std::env::args().nth(1).unwrap()
    }else if std::env::args().count() > 2{
        panic!("Too many arguments passed in.");
    }else{
        panic!("No file path provided.");
    };
    
    let mut app = Application::new()?;
    if let Err(e) = app.run(file_path){
        app.restore_terminal()?;
        println!("Encountered an error while running nlo code editor. error: {e}");
        return Err(e);
    }

    //let mut args: Vec<String> = std::env::args().skip(1).collect();
    //if let Some(file_path) = args.pop(){
    //    if let Err(e) = app.run(file_path){
    //        app.restore_terminal()?;
    //        println!("Encountered an error while running nlo text editor client. error: {}", e);
    //        return Err(e);
    //    }
    //}else{
    //    app.restore_terminal()?;
    //    println!("Could not open file. No file path provided.");
    //    return Ok(());
    //}

    app.restore_terminal()?;
    
    Ok(())
}
