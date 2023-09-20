extern crate db;
extern crate log;

use db::repository::Repository;

mod services;
mod views;

use services::Terminal;
use std::error::Error;

use crate::views::MainView;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    services::log::init()?;
    let repo = db::repository::CounterRespoitory;
    let _ = repo.first().await?;
    log::info!("Testing game logic: {}", game::count(1));

    let mut main_view = MainView::default();
    let mut terminal = Terminal::new()?;
    terminal.run(&mut main_view)?;
    terminal.restore()?;
    Ok(())
}
