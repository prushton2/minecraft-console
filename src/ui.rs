use tui::{
    Frame, backend::Backend, layout::{Alignment, Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Paragraph, Wrap}
};

pub struct UIState {
    pub players: Vec<String>,
    pub logs: Vec<String>,
    pub chat: Vec<String>,
    pub message_box: String,
    pub stdout: String,
    pub horizontal_scroll: u16,
    pub vertical_scroll: u16
}

pub fn ui<B: Backend>(f: &mut Frame<B>, state: &UIState) {
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref()
        )
        .split(f.size());

    let left_inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Max(u16::MAX),
                Constraint::Length(3)
            ].as_ref()
        )
        .split(chunks[0]);

    let right_inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ].as_ref()
        )
        .split(chunks[1]);

    let logs = create_scrolling_paragraph_block(
        "Logs".to_owned(),
        &state.logs.join("\n"),
        &left_inner_chunks[0],
        state.horizontal_scroll, state.vertical_scroll
    );
    
    let chat = create_scrolling_paragraph_block(
        "Chat".to_owned(),
        &state.chat.join("\n"),
        &left_inner_chunks[1],
        state.horizontal_scroll, state.vertical_scroll
    );
    

    let player_list_block = Block::default()
        .title("Players")
        .borders(Borders::ALL);
    let player_list = Paragraph::new(state.players.join("\n"))
        .block(player_list_block)
        .alignment(Alignment::Left);

    let message_out_block = Block::default()
        .title("Command Output")
        .borders(Borders::ALL);
    let message_out = Paragraph::new(state.stdout.clone())
        .block(message_out_block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);


    let message_block = Block::default()
        .title("")
        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);
    let mut message = state.message_box.clone();
    message.push('▏');
    let message = Paragraph::new(message)
        .block(message_block)
        .alignment(Alignment::Left);

    f.render_widget(player_list, right_inner_chunks[0]);
    f.render_widget(message_out, right_inner_chunks[1]);
    f.render_widget(logs, left_inner_chunks[0]);
    f.render_widget(chat, left_inner_chunks[1]);
    f.render_widget(message, left_inner_chunks[2]);
}

fn create_scrolling_paragraph_block(title: String, content: &String, area: &Rect, horizontal_scroll: u16, vertical_scroll: u16) -> Paragraph<'static> {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    let block_lines = block.inner(*area).height as i32;
    let mut content_lines: i32 = 1;
    content.chars().for_each(|c| {if c == '\n' { content_lines += 1;} else {}});

    let chat_scroll_offset = ((content_lines - block_lines) - vertical_scroll as i32).max(0);
    
    let chat = Paragraph::new(content.clone())
        .block(block)
        // .wrap(Wrap { trim: false })
        .scroll((chat_scroll_offset as u16, horizontal_scroll))
        .alignment(Alignment::Left);

    return chat
}