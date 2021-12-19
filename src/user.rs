extern crate termion;

use termion::{
    screen::*,
    cursor::{Hide, Show},
    raw::IntoRawMode,
    async_stdin,
    event::{parse_event, Key, Event},
};
use std::{
    io::{Read, Write, stdout},
    time::SystemTime,
};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Rect},
    buffer::Buffer,
    widgets::Widget,
    style::{Style, Color},
    widgets::{Block, BorderType, Borders},
    terminal::Terminal,
};
use crate::freecell::Freecell;
use deckofcards::*;

enum State {
    Dealing,
    Playing,
}

fn suit_to_uc_fill(suit: &Suit) -> char {
    match suit {
        deckofcards::Suit::Hearts => '♥',
        deckofcards::Suit::Spades => '♠',
        deckofcards::Suit::Diamonds => '♦',
        deckofcards::Suit::Clubs => '♣',
    }
}

fn show_card(card: &Card) -> String {
    format!("{}{}", card.rank.to_char(), suit_to_uc_fill(&card.suit))
}

fn style_card(card: &Card) -> Style {
    if card.is_hearts() || card.is_diamonds() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Black)
    }
}

fn show_card_or(buf: &mut Buffer, hoff: u16, voff: u16, maybe: Option<&Card>, def: &str) {
    match maybe {
        Some(card) => buf.set_string(hoff, voff, show_card(card), style_card(card)),
        None => buf.set_string(hoff, voff, def, Style::default().fg(Color::Black)),
    }
}

impl Widget for &Freecell {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let blockstyle = Style::default().bg(Color::Green).fg(Color::Black);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Rust FreeCell #{}",self.seed))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(blockstyle);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Show homecells and freecels
        let filler = "[]";

        for i in 0..4 {
            // homecell
            let hoff = 2 + (i * 3);
            show_card_or(buf, hoff, inner_area.top(), self.get_homecell_topcard(i as usize), &filler);
            // freecell
            let hoff = 21 + (i * 3);
            show_card_or(buf, hoff, inner_area.top(), self.get_freecell_card(i as usize), &filler);
        }

        // Show crown or top card
        match self.deck.top_card() {
            Some(card) => buf.set_string(16, inner_area.top(), show_card(&card), style_card(&card)),
            None => buf.set_string(16, inner_area.top(), String::from("👑"), Style::default())
        }

        let mut height = self.minheight;
        for i in 0..8 {
            let l = self.field[i].len();
            if l > height {
                height = l
            }
        }

        for depth in 0..height {
            let filler = "  ";
            let voff = inner_area.top() as usize + 1 + depth;
            for field in 0..8 {
                let hoff = 2 + (field * 4);
                show_card_or(buf, hoff as u16, voff as u16, self.get_field_card_at(field, depth), &filler);
            }
        }
    }
}

pub fn run(mut board: Freecell) {
    let mut stdin = async_stdin().bytes();
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let backend = TermionBackend::new(screen);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut darkmode = false;
    let mut state = State::Dealing;

    let mut last_time = SystemTime::now();

    let tick_length = 50;

    write!(terminal.backend_mut(), "{}", Hide).ok();

    loop {
        match &state {
            State::Dealing => {
                if SystemTime::now().duration_since(last_time).ok().unwrap().as_millis() > tick_length {
                    last_time = SystemTime::now();
                    if ! board.deal_one() {
                        state = State::Playing;
                    }
                }
            },
            State::Playing => (),
        }

        let blockstyle = if darkmode {
            Style::default().bg(Color::Rgb(0, 64, 0)).fg(Color::Gray)
        } else {
            Style::default().bg(Color::Green).fg(Color::Black)
        };

        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .borders(Borders::ALL)
                .title(format!("Rust FreeCell #{}",board.seed))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(blockstyle);
            f.render_widget(block, size);

            f.render_widget(&board, size);
        }).unwrap();

        if let Some(Ok(nextbyte)) = stdin.next() {
            if let Ok(Event::Key(event)) = parse_event(nextbyte, &mut stdin) {
                match event {
                    Key::Char('q') => break,
                    Key::Char('d') => { board.deal_one(); }
                    Key::Char('m') => darkmode = !darkmode,
                    Key::Char(c) => println!("{}\r", c),
                    _ => println!("special\r"),
                }
            }
        }
    }

    write!(terminal.backend_mut(), "{}", Show).ok();
}