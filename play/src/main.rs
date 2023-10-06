extern crate db;
extern crate game;
extern crate log;

mod services;
mod terminal;
mod utils;
mod view_models;
mod views;

use db::repository::Repository;
use std::error::Error;
use terminal::Terminal;

use crate::view_models::MainViewModel;
use crate::views::MainView;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    services::log::init()?;
    let repo = db::repository::CounterRespoitory;
    let _ = repo.first().await?;
    log::info!("Testing game logic: {}", game::count(1));

    let mut main_vm = MainViewModel::new(&repo);
    let mut main_view = MainView::new();
    let mut terminal = Terminal::new()?;
    terminal.run(&mut main_view, &mut main_vm).await?;
    terminal.restore()?;
    Ok(())
}
