use tui::{
    Frame, backend::Backend, layout::{Alignment, Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph, Wrap}
};

pub struct UIState<'a> {
    pub players: Vec<&'a str>,
    pub logs: Vec<&'a str>,
    pub chat: Vec<&'a str>,
    pub message_box: String
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

    let inner_chunks = Layout::default()
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

    
    let player_list_block = Block::default()
        .title("Players")
        .borders(Borders::ALL);

    let player_list = Paragraph::new(state.players.join("\n"))
        .block(player_list_block)
        .alignment(Alignment::Left);


    let chat_block = Block::default()
        .title("Chat")
        .borders(Borders::ALL);

    let chat_block_lines = chat_block.inner(inner_chunks[0]).height as i16;
    let chat_scroll_offset = (state.logs.len() as i16 - chat_block_lines).max(0);


    let chat = Paragraph::new(state.chat.join("\n"))
        .block(chat_block)
        .wrap(Wrap { trim: false }).
        scroll((chat_scroll_offset as u16, 0))
        .alignment(Alignment::Left);


    let logs_block = Block::default()
        .title("Logs")
        .borders(Borders::ALL);

    let logs_block_lines = logs_block.inner(inner_chunks[0]).height as i16;
    let logs_scroll_offset = (state.logs.len() as i16 - logs_block_lines).max(0);

    let logs = Paragraph::new(state.logs.join("\n"))
        .block(logs_block)
        .wrap(Wrap { trim: false })
        .scroll((logs_scroll_offset as u16, 0))
        .alignment(Alignment::Left);

    let message_block = Block::default()
        .title("")
        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);

    let mut message = state.message_box.clone();
    message.push('▏');

    let message = Paragraph::new(message)
        .block(message_block)
        .alignment(Alignment::Left);

    f.render_widget(player_list, chunks[1]);
    f.render_widget(logs, inner_chunks[0]);
    f.render_widget(chat, inner_chunks[1]);
    f.render_widget(message, inner_chunks[2]);
}