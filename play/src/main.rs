extern crate db;
extern crate log;

mod services;
// mod view_models;
mod views;

use db::repository::Repository;
use services::Terminal;
use std::error::Error;

// use crate::view_models::MainViewModel;
use crate::views::MainView;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    services::log::init()?;
    let repo = db::repository::CounterRespoitory;
    let _ = repo.first().await?;
    log::info!("Testing game logic: {}", game::count(1));

    // let mut main_viewmodel = MainViewModel::new(&mut repo);
    let mut main_view = MainView::default();
    let mut terminal = Terminal::new()?;
    terminal.run(&mut main_view)?;
    terminal.restore()?;
    Ok(())
}
