use ratatui::prelude::*;
use std::error::Error;

use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::terminal::Frame;
use crate::view_models::CounterVm;

pub struct CounterView;

impl CounterView {
    pub fn new() -> Self {
        Self
    }

    fn size(&self, f: Rect) -> Rect {
        let width = 5 + 5 + 5;
        let height = 5 + 5 + 5;
        Rect {
            x: f.width / 2 - width / 2,
            y: f.height / 2 - height / 2,
            width,
            height,
        }
    }

    pub fn draw(&self, f: &mut Frame, vm: &dyn CounterVm) {
        let items = vec![
            "Counter".to_owned(),
            vm.data().name.clone(),
            vm.data().count.to_string(),
        ];
        let list_items: Vec<ListItem> = items.iter().cloned().map(ListItem::new).collect();
        let list =
            List::new(list_items).block(Block::default().title("Main Menu").borders(Borders::ALL));
        f.render_widget(list, self.size(f.size()));
    }

    pub async fn handle_key(
        &self,
        code: KeyCode,
        vm: &mut dyn CounterVm,
    ) -> Result<(), Box<dyn Error>> {
        match code {
            KeyCode::Char('s') => vm.save().await,
            KeyCode::Char('l') => vm.load().await,
            KeyCode::Char('a') => {
                vm.add();
                Ok(())
            }
            KeyCode::Char('m') => {
                vm.minus();
                Ok(())
            }
            KeyCode::Char('r') => {
                vm.reset();
                Ok(())
            }
            KeyCode::Char(c) => {
                log::error!("Counter View: unhandled key: {}", c);
                Ok(())
            }
            _ => {
                log::error!("Counter View: non char key detected");
                Ok(())
            }
        }
    }
}
