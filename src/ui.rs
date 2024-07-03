use crate::application::{AppState, Mode, WarningKind};
use nlo_text_editor_server::Position;
use std::error::Error;
use ratatui::Terminal;
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::style::{Style, Color, Stylize};
use ratatui::layout::{Alignment, Direction, Layout, Constraint};



const DEFAULT_DOCUMENT_TEXT: &str = "Nlo Text Editor\n\nNo Document Open";



pub struct UserInterface{
    terminal_size: Rect,
    //offset: Position, // handle offset server side?
    /// the area of the terminal filled by an open document
    document_rect: Rect,
    text_in_view: String,
    document_open: bool,
    client_cursor_position: Option<Position>,
}
impl UserInterface{
    pub fn new(terminal_size: Rect) -> Self{
        Self{
            terminal_size,
            //offset: Position::default(),
            document_rect: Rect::default(),
            text_in_view: String::new(),
            document_open: false,
            client_cursor_position: None,
        }
    }
    pub fn set_terminal_size(&mut self, width: u16, height: u16){
        self.terminal_size.width = width;
        self.terminal_size.height = height;
    }

    pub fn document_rect(&self) -> Rect{
        self.document_rect
    }

    pub fn set_text_in_view(&mut self, text: String){
        self.text_in_view = text;
    }

    pub fn set_document_open(&mut self, val: bool){
        self.document_open = val;
    }

    pub fn set_client_cursor_position(&mut self, position: Option<Position>){
        self.client_cursor_position = position;
    }


    //pub fn update_layouts(&mut self, app: &AppState, editor: &Editor){
    pub fn update_layouts(&mut self){
        // layout of viewport rect (the whole terminal screen)
        let viewport_rect = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                vec![
                    // document + line num rect height
                    Constraint::Min(0),
                ]
            )
            .split(self.terminal_size);

        self.document_rect = viewport_rect[0];
    }

    pub fn document_widget(&self) -> Paragraph<'static>{
        if self.document_open{
            Paragraph::new(self.text_in_view.clone())
                //.scroll((self.offset.y() as u16, self.offset.x() as u16)) // we will be handling view scrolling in the server
        }else{
            Paragraph::new(DEFAULT_DOCUMENT_TEXT)
                .alignment(Alignment::Center)
                .red()
        }
    }

    // when in select mode, figure out how to change background color of text within cursor_head and cursor_anchor
    //pub fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &AppState, editor: &Editor) -> Result<(), Box<dyn Error>>{
    pub fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &AppState) -> Result<(), Box<dyn Error>>{
        // testing calling this here, instead of from main.rs
        self.update_layouts();
        //
        
        terminal.draw(
            |frame| {

                // render widgets
                frame.render_widget(self.document_widget(), self.document_rect);

                // render cursor
                match app.mode(){
                    Mode::Insert => {
                        //if let Some(doc) = editor.document(){
                        //    frame.set_cursor(
                        //        self.document_rect.x + doc.cursor_position().x()
                        //            .saturating_sub(self.offset.x()) as u16
                        //        , 
                        //        self.document_rect.y + doc.cursor_position().y()
                        //            .saturating_sub(self.offset.y()) as u16
                        //    )
                        //}
                        if let Some(pos) = self.client_cursor_position{
                            frame.set_cursor(pos.x() as u16, pos.y() as u16)
                        }
                    }
                    _ => {}
                }
            }
        )?;

        Ok(())
    }
}

//fn _centered_rect(percent_x: u16, percent_y: u16, r: ratatui::prelude::Rect) -> ratatui::prelude::Rect{
//    let popup_layout = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(
//            [
//                Constraint::Percentage((100 - percent_y) / 2),
//                Constraint::Percentage(percent_y),
//                Constraint::Percentage((100 - percent_y) / 2),
//            ]
//            .as_ref(),
//        )
//        .split(r);
//
//    Layout::default()
//        .direction(Direction::Horizontal)
//        .constraints(
//            [
//                Constraint::Percentage((100 - percent_x) / 2),
//                Constraint::Percentage(percent_x),
//                Constraint::Percentage((100 - percent_x) / 2),
//            ]
//            .as_ref(),
//        )
//        .split(popup_layout[1])[1]
//}