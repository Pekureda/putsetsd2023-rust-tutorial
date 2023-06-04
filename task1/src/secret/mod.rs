//DO NOT MODIFY THIS FILE

use std::{thread, time, io::{Stdout,Write}};
use terminal::{Terminal,Action, Event, KeyCode, Retrieved, Color};
mod draw;

pub fn blackjack(){
    let mut term = terminal::stdout();
    let mut dealer_cards:Vec<u8>=Vec::new();
    let mut player_cards:Vec<u8>=Vec::new();


    term.act(Action::SetForegroundColor(terminal::Color::Cyan)).unwrap();
    term.write("New game of blackjack starts".as_bytes()).unwrap();
    term.act(Action::ResetColor).unwrap();
    //Delear draws 1 card and Player draws 2 cards
    term.act(Action::SetForegroundColor(terminal::Color::Red)).unwrap();
    draw_and_print(&mut term, "Dealer", &mut dealer_cards);
    term.act(Action::ResetColor).unwrap();
    for _i in 1..=2{
        draw_and_print(&mut term, "Player", &mut player_cards);
    }

    //Hanlde player decisions
    player_decisions(&mut term, &mut player_cards);

    //Dealer draws as long as his sum is less than 17
    term.act(Action::SetForegroundColor(terminal::Color::Red)).unwrap();
    loop{
        draw_and_print(&mut term, "Dealer", &mut dealer_cards);
        if dealer_cards.iter().sum::<u8>() >= 17{
            break;
        }
    }
    term.act(Action::ResetColor).unwrap();
    //Check who wins
    let dealer_sum=dealer_cards.iter().sum::<u8>();
    let player_sum=player_cards.iter().sum::<u8>();
    let scoreboard = format!("{}\nPlayer's total: {}\nDealer's total: {}\n", "=".repeat(80), player_sum, dealer_sum);
    if player_sum <= 21 && (player_sum > dealer_sum || dealer_sum > 21){
        surprise(&mut term, &scoreboard);
    }else{
        println!("{scoreboard}You lose!");
    }
}


fn player_decisions(term: &mut Terminal<Stdout>, player_cards: &mut Vec<u8>){
    term.act(Action::SetForegroundColor(terminal::Color::Cyan)).unwrap();
    term.write("Press h to hit and draw a card\nor s to stay and end your turn\n".as_bytes()).unwrap();
    term.act(Action::ResetColor).unwrap();
    loop{
        if player_cards.iter().sum::<u8>() >= 21{
            break;
        }
        match term.get(terminal::Value::Event(None)).unwrap(){
            Retrieved::Event(o) => {
                match o.unwrap(){
                    Event::Key(key_event) => {
                        match key_event.code{
                            KeyCode::Char('h') =>{
                                draw_and_print(term, "Player", player_cards)
                            },
                            KeyCode::Char('s') =>{
                                break;
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            _ => (),

        }
    }
}
fn draw_and_print(term: &mut Terminal<Stdout>, who:&str, cards: &mut Vec<u8>){
    let card = draw::draw_card();
    cards.push(card);
    term.write(format!("{} draws {} and has total of {}\n", who, card, cards.iter().sum::<u8>()).as_bytes()).unwrap();
    thread::sleep(time::Duration::from_millis(500));
}

//THIS CODE WAS TAKEN FROM https://github.com/slightknack/rusty-donut
//IT WAS SLIGHTLY MODIFIED
fn surprise(term: &mut Terminal<Stdout>, scoreboard: &str) {
    let (mut a, mut b) = (1.0_f64, 1.0_f64);
    let now = time::Instant::now();
    term.act(terminal::Action::ClearTerminal(terminal::Clear::All)).unwrap();
    term.act(terminal::Action::HideCursor).unwrap();
    let mut counter: usize=0;
    let dance = [" ┗(･o･)┓", " ┏(･o･)┛"];
    let colors = [Color::Blue,Color::DarkBlue, Color::Red, Color::DarkRed, Color::Cyan, Color::Magenta, Color::Yellow, Color::DarkYellow, Color::Green, Color::DarkGreen];
    loop {
        a += 0.07;
        b += 0.03;
        let ((g, m), (p, r), mut q, mut z, mut j) = (
            a.sin_cos(),
            b.sin_cos(),
            [' '; 1760],
            [0.0_f64; 1760],
            0.0_f64,
        );
        while j <= std::f64::consts::TAU {
            let (u, v) = j.sin_cos();
            let mut i = 0.0f64;
            while i <= std::f64::consts::TAU {
                let (w, c) = i.sin_cos();
                let h = v + 2.0;
                let (d, t) = (1.0 / (w * h * g + u * m + 5.0), w * h * m - u * g);
                let (x, y) = (
                    (40.0 + 30.0 * d * (c * h * r - t * p)) as usize,
                    (12.0 + 15.0 * d * (c * h * p + t * r)) as usize,
                );
                let (o, n) = (
                    x + 80 * y,
                    8.0 * ((u * g - w * v * m) * r - w * v * g - u * m - c * v * p),
                );
                if y < 22 && x < 79 && d > z[o] {
                    z[o] = d;
                    q[o] = ("▁▂▂▃▄▄▅".to_owned() + "▆▆▇██")
                        .chars()
                        .nth(n as usize)
                        .or(Some('▁'))
                        .unwrap();
                }
                i += 0.02
            }
            j += 0.07
        }
        term.batch(terminal::Action::MoveCursorTo(0, 0)).unwrap();
        term.batch(Action::SetForegroundColor(colors[(counter/5)%10])).unwrap();
        term.write(format!("{scoreboard}You win!\nTime to PARTY!\n").as_bytes()).unwrap();
        let dance_line = dance[(counter/10)%2].repeat(10);
        term.write(dance_line.as_bytes()).unwrap();
        term.write(
            format!("\n{}\n",
                q.chunks(80)
                    .map(|l| l.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n")
            ).as_bytes()
        ).unwrap();
        term.write(dance_line.as_bytes()).unwrap();
        term.flush_batch().unwrap();
        thread::sleep(time::Duration::from_millis(16));
        let elapsed = now.elapsed();
        if elapsed.as_millis() > 8000 {
            term.batch(Action::ResetColor).unwrap();
            term.batch(terminal::Action::ClearTerminal(terminal::Clear::All)).unwrap();
            term.batch(terminal::Action::MoveCursorTo(0, 0)).unwrap();
            term.batch(terminal::Action::ShowCursor).unwrap();
            term.flush_batch().unwrap();
            break;
        }
        counter+=1;
    }
} /* Based on donut.c */
/* by Andy Sloane. Translated to */
/* Rust by Isaac Clayton- */
/* @slightknack */