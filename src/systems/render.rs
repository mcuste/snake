//! Rendering systems.

use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::{
    components::{Food, GridPos, SnakeHead, SnakeSegment},
    config::{GRID_HEIGHT, GRID_HEIGHT_U16, GRID_WIDTH, GRID_WIDTH_U16},
    resources::{Score, TerminalSize},
};

const BOARD_COLS: usize = GRID_WIDTH as usize;
const BOARD_ROWS: usize = GRID_HEIGHT as usize;
/// Width of the game-over overlay box in terminal columns.
const OVERLAY_W: u16 = 26;
/// Height of the game-over overlay box in terminal rows.
const OVERLAY_H: u16 = 5;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Head,
    Body,
    Food,
}

struct GameBoard([[Cell; BOARD_COLS]; BOARD_ROWS]);

impl Widget for GameBoard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (row_idx, row) in self.0.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let (Ok(r), Ok(c)) = (u16::try_from(row_idx), u16::try_from(col_idx)) {
                    let (ch, style) = match cell {
                        Cell::Head => ("@", Style::default().fg(Color::LightGreen)),
                        Cell::Body => ("O", Style::default().fg(Color::Green)),
                        Cell::Food => ("*", Style::default().fg(Color::LightRed)),
                        Cell::Empty => (" ", Style::default()),
                    };
                    buf.set_string(area.x + c, area.y + r, ch, style);
                }
            }
        }
    }
}

/// Writes `value` into `grid` at the cell corresponding to `pos`, ignoring out-of-range positions.
fn set_cell(grid: &mut [[Cell; BOARD_COLS]; BOARD_ROWS], pos: GridPos, value: Cell) {
    if let (Ok(x), Ok(y)) = (usize::try_from(pos.x), usize::try_from(pos.y)) {
        if let Some(row) = grid.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                *cell = value;
            }
        }
    }
}

/// Update (`GameSet::Render`): draws the board and snake in the `Playing` state.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy system parameters must be owned; Res<T> is not Clone/Copy"
)]
pub(crate) fn render(
    mut ctx: ResMut<'_, RatatuiContext>,
    head_query: Query<'_, '_, &GridPos, With<SnakeHead>>,
    segment_query: Query<'_, '_, &GridPos, With<SnakeSegment>>,
    food_query: Query<'_, '_, &GridPos, With<Food>>,
    terminal_size: Res<'_, TerminalSize>,
    score: Res<'_, Score>,
) {
    let mut grid = [[Cell::Empty; BOARD_COLS]; BOARD_ROWS];
    for pos in &segment_query {
        set_cell(&mut grid, *pos, Cell::Body);
    }
    for pos in &food_query {
        set_cell(&mut grid, *pos, Cell::Food);
    }
    // Head written last so it overwrites body/food if they coincide.
    for pos in &head_query {
        set_cell(&mut grid, *pos, Cell::Head);
    }

    let board_w = GRID_WIDTH_U16 + 2;
    let board_h = GRID_HEIGHT_U16 + 2;
    let offset_x = terminal_size.cols.saturating_sub(board_w) >> 1;
    let offset_y = terminal_size.rows.saturating_sub(board_h) >> 1;
    let score_val = score.0;

    let _ = ctx.draw(|frame| {
        let outer = Rect::new(offset_x, offset_y, board_w, board_h);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Score: {score_val} "));
        let inner = block.inner(outer);
        frame.render_widget(block, outer);
        frame.render_widget(GameBoard(grid), inner);
    });
}

/// Update (`GameSet::Render`): draws the stale board with game-over overlay.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy system parameters must be owned; Res<T> is not Clone/Copy"
)]
pub(crate) fn game_over_render(
    mut ctx: ResMut<'_, RatatuiContext>,
    score: Res<'_, Score>,
    head_query: Query<'_, '_, &GridPos, With<SnakeHead>>,
    segment_query: Query<'_, '_, &GridPos, With<SnakeSegment>>,
    food_query: Query<'_, '_, &GridPos, With<Food>>,
    terminal_size: Res<'_, TerminalSize>,
) {
    let mut grid = [[Cell::Empty; BOARD_COLS]; BOARD_ROWS];
    for pos in &segment_query {
        set_cell(&mut grid, *pos, Cell::Body);
    }
    for pos in &food_query {
        set_cell(&mut grid, *pos, Cell::Food);
    }
    for pos in &head_query {
        set_cell(&mut grid, *pos, Cell::Head);
    }

    let board_w = GRID_WIDTH_U16 + 2;
    let board_h = GRID_HEIGHT_U16 + 2;
    let offset_x = terminal_size.cols.saturating_sub(board_w) >> 1;
    let offset_y = terminal_size.rows.saturating_sub(board_h) >> 1;
    let score_val = score.0;

    let _ = ctx.draw(|frame| {
        let outer = Rect::new(offset_x, offset_y, board_w, board_h);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Score: {score_val} "));
        let inner = block.inner(outer);
        frame.render_widget(block, outer);
        frame.render_widget(GameBoard(grid), inner);

        // Overlay: ~26 wide x 5 tall, centered within the inner board area.
        let overlay_x = inner.x + (inner.width.saturating_sub(OVERLAY_W) >> 1);
        let overlay_y = inner.y + (inner.height.saturating_sub(OVERLAY_H) >> 1);
        let overlay = Rect::new(overlay_x, overlay_y, OVERLAY_W, OVERLAY_H);
        frame.render_widget(Clear, overlay);

        let text = Text::from(vec![
            Line::from(Span::styled(
                "GAME OVER",
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("Score: {score_val}"),
                Style::default(),
            )),
            Line::raw(""),
            Line::from(Span::styled(
                "r = restart   q = quit",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        frame.render_widget(paragraph, overlay);
    });
}
