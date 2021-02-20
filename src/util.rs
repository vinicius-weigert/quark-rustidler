use tui::{
    layout::{
        Rect,
        Layout,
        Direction,
        Constraint
    }
};

pub fn centered_rect(x_percent: u16, y_percent: u16, base_rect: Rect) -> Vec<Rect> {
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100-x_percent)/2),
                Constraint::Percentage(x_percent),
                Constraint::Percentage((100-x_percent)/2),
            ]
            .as_ref(),
        )
        .split(base_rect);

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100-y_percent)/2),
                Constraint::Percentage(y_percent),
                Constraint::Percentage((100-y_percent)/2),
            ]
            .as_ref(),
        )
        .split(h_layout[1]);

    v_layout
}
