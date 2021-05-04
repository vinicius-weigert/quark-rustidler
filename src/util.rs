use tui::{
    layout::{
        Rect,
        Layout,
        Direction,
        Constraint
    }
};

pub fn centered_rect(x_percent: i16, y_percent: i16, base_rect: Rect) -> Vec<Rect> {
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(((100-x_percent)/2).abs() as u16),
                Constraint::Percentage(x_percent.abs() as u16),
                Constraint::Percentage(((100-x_percent)/2).abs() as u16),
            ]
            .as_ref(),
        )
        .split(base_rect);

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(((100-y_percent)/2).abs() as u16),
                Constraint::Percentage(y_percent.abs() as u16),
                Constraint::Percentage(((100-y_percent)/2).abs() as u16),
            ]
            .as_ref(),
        )
        .split(h_layout[1]);

    v_layout
}

pub fn offset_rect(percent: (i16, i16), offset: (i16, i16), base_rect: Rect) -> Vec<Rect> {
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(((100-percent.0+offset.0)/2).abs() as u16),
                Constraint::Percentage(percent.0.abs() as u16),
                Constraint::Percentage(((100-percent.0-offset.0)/2).abs() as u16),
            ]
            .as_ref(),
        )
        .split(base_rect);

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(((100-percent.1+offset.1)/2).abs() as u16),
                Constraint::Percentage((percent.1).abs() as u16),
                Constraint::Percentage(((100-percent.1-offset.1)/2).abs() as u16),
            ]
            .as_ref(),
        )
        .split(h_layout[1]);

    v_layout
}
