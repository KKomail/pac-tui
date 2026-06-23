use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
};

use crate::app::{App, Tab};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_tabs(frame, app, chunks[0]);
    render_content(frame, app, chunks[1]);
    render_status_bar(frame, app, chunks[2]);
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::titles()
        .iter()
        .map(|t| Line::from(Span::raw(*t)))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" pac-tui "))
        .select(app.current_tab.index())
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

fn render_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.current_tab {
        Tab::Packages => render_packages(frame, app, area),
        Tab::Search => render_placeholder(frame, area, " Search ", "Search not yet implemented"),
        Tab::Updates => render_placeholder(
            frame,
            area,
            " Updates ",
            "Update checking not yet implemented",
        ),
    }
}

fn render_packages(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(err) = &app.load_error {
        let msg = Paragraph::new(format!("Error loading packages: {err}"))
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Installed Packages "),
            );
        frame.render_widget(msg, area);
        return;
    }

    let show_search_bar = app.searching || !app.search_query.is_empty();

    let (content_area, search_area) = if show_search_bar {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    if app.show_details {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(content_area);
        render_package_list(frame, app, chunks[0]);
        render_package_details(frame, app, chunks[1]);
    } else {
        render_package_list(frame, app, content_area);
    }

    if let Some(area) = search_area {
        render_search_bar(frame, app, area);
    }
}

fn render_package_list(frame: &mut Frame, app: &App, area: Rect) {
    let visible = app.visible_packages();

    let name_col_width = visible
        .iter()
        .map(|p| p.name.len())
        .max()
        .unwrap_or(20)
        .min(40);

    let items: Vec<ListItem> = visible
        .iter()
        .map(|p| {
            let name = format!("{:<width$}", p.name, width = name_col_width);
            let line = Line::from(vec![
                Span::styled(name, Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled(p.version.clone(), Style::default().fg(Color::DarkGray)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let mut state = ListState::default();
    if !visible.is_empty() {
        state.select(Some(app.selected_index));
    }

    let title = if app.search_query.is_empty() {
        format!(" Installed Packages ({}) ", app.packages.len())
    } else {
        format!(" Installed Packages ({}/{}) ", visible.len(), app.packages.len())
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let query = &app.search_query;
    let border_style = if app.searching {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let bar = Paragraph::new(format!("/{query}"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Search ")
                .border_style(border_style),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(bar, area);

    if app.searching {
        frame.set_cursor_position((
            area.x + 2 + query.len() as u16,
            area.y + 1,
        ));
    }
}

fn render_package_details(frame: &mut Frame, app: &App, area: Rect) {
    let name = app
        .packages
        .get(app.selected_index)
        .map(|p| p.name.as_str())
        .unwrap_or("Package");

    let title = format!(" {name} — Details ");

    let content = match &app.package_details {
        Some(lines) => lines.join("\n"),
        None => String::from("Loading..."),
    };

    let detail = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(detail, area);
}

fn render_placeholder(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    let p = Paragraph::new(message).block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(p, area);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let key = |label: &'static str| {
        Span::styled(
            label,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    };

    let spans = if app.searching {
        vec![
            key(" type"),
            Span::raw(":filter  "),
            key("enter"),
            Span::raw(":confirm  "),
            key("backspace"),
            Span::raw(":delete  "),
            key("esc"),
            Span::raw(":cancel"),
        ]
    } else {
        vec![
            key(" q"),
            Span::raw(":quit  "),
            key("tab/shift-tab"),
            Span::raw(":switch tab  "),
            key("↑/k  ↓/j"),
            Span::raw(":navigate  "),
            key("g/G"),
            Span::raw(":first/last  "),
            key("enter"),
            Span::raw(":details  "),
            key("/"),
            Span::raw(":search  "),
            key("esc"),
            Span::raw(":close"),
        ]
    };

    let bar = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(bar, area);
}
