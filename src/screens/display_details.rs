pub use crate::prelude::*;

pub fn draw_display_details(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    app: &mut App,
    keys: &Arc<Mutex<Vec<String>>>,
    details: Vec<Spans>,
) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(6), Constraint::Percentage(94)].as_ref())
        .split(f.size());

    let search_and_tabs = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(main_layout[0]);

    let result_preview_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(main_layout[1]);

    let mut search = Paragraph::new(String::from(&app.input))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Search"));

    let mut title_bar = Block::default().title("Title bar").borders(Borders::ALL);

    app.set_num_showed_elements(keys.lock().unwrap().len());
    let mut results_widget = app.draw_results::<String>(keys.lock().unwrap().clone());
    results_widget = results_widget.block(Block::default().borders(Borders::ALL));

    let mut data =
        Paragraph::new(details).block(Block::default().title("Data").borders(Borders::ALL));

    let color = if app.element_selected.clone() {
        Color::Red
    } else {
        Color::Yellow
    };

    match app.highlighted_element {
        WindowElements::Search => {
            search = search.style(Style::default().fg(color));
        }
        WindowElements::TitleBar => {
            title_bar = title_bar.style(Style::default().fg(color));
        }
        WindowElements::Results => {
            results_widget = results_widget.style(Style::default().fg(color));
        }
        WindowElements::Data => {
            data = data.style(Style::default().fg(color));
        }
    }

    f.render_widget(search, search_and_tabs[0]);
    f.render_widget(title_bar, search_and_tabs[1]);
    f.render_stateful_widget(
        results_widget,
        result_preview_layout[0],
        &mut app.result_state.clone(),
    );
    f.render_widget(data, result_preview_layout[1])
}

pub fn generate_display_text(value: String) -> Vec<Spans<'static>> {
    vec![Spans::from(vec![Span::raw(value)])]
}
