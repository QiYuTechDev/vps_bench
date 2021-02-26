use std::io;

use tui::backend::CrosstermBackend;
use tui::widgets;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    println!("hello");
    terminal
        .draw(|f| {
            let size = f.size();
            let block = {
                let block = widgets::Block::default();
                block.title("Rust TUI")
            };
            f.render_widget(block, size);
        })
        .unwrap();
    terminal.flush().unwrap();
    terminal.clear().unwrap();
    drop(terminal);
    println!("world");
    Ok(())
}
