mod main;
pub use main::View as Main;

use ratatui::prelude::*;

pub const HIGHLIGHT_COLOR: Color = Color::Red;

fn title(text: &str, highlight: usize) -> Vec<Span<'_>> {
    let pre = &text[..highlight];
    let letter: char = text.chars().nth(highlight).unwrap_or_default();
    let post = &text[highlight + 1..];

    vec![
        Span::from(" "),
        Span::from(pre),
        Span::styled(letter.to_string(), Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)),
        Span::from(post),
        Span::from(" "),
    ]
}