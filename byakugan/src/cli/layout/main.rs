use ratatui::{prelude::*, widgets::{Block, Borders}};

pub struct View;
impl View {
    pub fn render(frame: &mut Frame) {
        // Vertical split
        /*
        +------------------------+
        |                        |
        |                        | >=20
        |                        |
        +------------------------+
        |                        | =10
        +------------------------+       
        */
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                ]
            )
            .split(frame.size());

        frame.render_widget(
            Block::new()
                // don't render the right border because it will be rendered by the right block
                .borders(Borders::ALL).border_set(symbols::border::PLAIN)
                .title(super::title("main", 0))
                .title_alignment(Alignment::Left),
            layout[0],
        );

        frame.render_widget(
            Block::new()
                .borders(Borders::TOP)
                    .border_set(symbols::border::PLAIN)
                .title(vec![
                    Span::from(" "),
                    Span::styled("i", Style::default().fg(super::HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)),
                    Span::from("mport ─ "),
                    Span::styled("s", Style::default().fg(super::HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)),
                    Span::from("ettings ─ "),
                    Span::styled("h", Style::default().fg(super::HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)),
                    Span::from("elp ─ "),
                    Span::styled("q", Style::default().fg(super::HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)),
                    Span::from("uit "),
                ])
                .title_alignment(Alignment::Center),
            layout[1],
        );
    }
}