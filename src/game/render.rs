use super::Game;
use super::State;
use crossterm::style::Print;
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
};
use std::io::BufWriter;
use std::io::StdoutLock;
use std::io::{Write, stdout};

pub fn render_game(game: &Game, display: (u16, u16)) {
    let mut out = BufWriter::new(stdout().lock());
    clear_screen(&mut out);

    if display.0 < 39 || display.1 < 7 {
        writeln!(&mut out, "Small Terminal").unwrap();
        out.flush().unwrap();
        return;
    }

    let over_screen = format!(
        "===============< Snake >===============\n|         Score :  {}{}         |\n|    High Score :  {}{}         |\n---------------------------------------\n|         <Space>  ==>  Play          |\n|              q   ==>  Exit          |\n|_____________________________________|",
        game.score,
        " ".repeat(10 - numlen(game.score)),
        game.highscore,
        " ".repeat(10 - numlen(game.highscore))
    );

    let paused_screen = format!(
        "===============< Paused >==============\n|         Score :  {}{}         |\n|    High Score :  {}{}         |\n---------------------------------------\n|         <Space>  ==>  Resume        |\n|         q        ==>  Reset         |\n|_____________________________________|",
        game.score,
        " ".repeat(10 - numlen(game.score)),
        game.highscore,
        " ".repeat(10 - numlen(game.highscore))
    );

    if game.state == State::Over || game.state == State::Paused {
        let col = (display.0 / 2) - 19;
        let mut row = (display.1 / 2) - 3;

        let screen_to_render = if game.state == State::Over {
            &over_screen
        } else {
            &paused_screen
        };

        for line in screen_to_render.lines() {
            crossterm::queue!(out, cursor::MoveTo(col, row), Print(line)).unwrap();

            row += 1;
        }
    } else {
        crossterm::queue!(out, Print(game.score)).unwrap();
        let body = "\x1b[103m \x1b[0m";
        let head = "\x1b[104m \x1b[0m";
        let food = "\x1b[102m \x1b[0m";
        for cell in &game.snake.body {
            crossterm::queue!(out, cursor::MoveTo(cell.0, cell.1), Print(body)).unwrap();
        }

        let h = &game.snake.body.get(game.snake.body.len() - 1).unwrap();
        crossterm::queue!(out, cursor::MoveTo(h.0, h.1), Print(head)).unwrap();

        crossterm::queue!(out, cursor::MoveTo(game.food.0, game.food.1), Print(food)).unwrap();
    }

    out.flush().unwrap();
}

fn clear_screen(out: &mut BufWriter<StdoutLock>) {
    crossterm::queue!(out, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

fn numlen(n: u32) -> usize {
    if n == 0 {
        return 1;
    }
    let mut n = n;
    let mut ln = 0;
    while n != 0 {
        n = n / 10;
        ln += 1;
    }
    ln
}
