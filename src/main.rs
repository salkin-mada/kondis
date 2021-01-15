use substring::Substring;
use cursive::Cursive;
use cursive::views::{Dialog, EditView, TextView};
use cursive::views;
#[allow(unused_imports)]
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
// use cursive::utils::Counter;
// use cursive::traits::With as _;

fn main() {
    let mut tui = cursive::default();
    // theme
    style(&mut tui);
    // mappings
    mappings(&mut tui);
    // interaction
    input(&mut tui);
}

fn input(tui: &mut Cursive) {
    tui.add_layer(
        Dialog::new()
        .title("< Kondis >")
        .padding_lrtb(1, 1, 1, 0)
        .content(
            EditView::new().filler(" ")
            .max_content_width(3)
            .on_submit(show_result)
            .with_name("val")
            .fixed_width(4)
        )
        .button("Calculate", |s| {
            let val = s
                .call_on_name("val", |view: &mut EditView| {
                    view.get_content()
                })
            .unwrap();
            show_result(s, &val);
        }),
    );
    tui.run();
}

#[allow(unused_variables)]
fn show_result(s: &mut Cursive, value: &str) {
    if value.is_empty() {
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new("Enter a value!"))
            .button("Ok", |s| {
                s.pop_layer();
                input(s);
            }));
    } else if value.len() < 3 {
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new("Enter 3 integers.."))
            .button("Ok", |s| {
                s.pop_layer();
                input(s);
            }));
    } else {
        let sub = value.substring(0,2);
        let val = f32::from(sub.parse::<f32>().unwrap());
        let mul_sub = value.substring(2,3);
        let mul = f32::from(mul_sub.parse::<f32>().unwrap());
        let ok_input = value.substring(0,3);
        let result = val / 1000f32;
        let insight = format!("mark: {}\nkondis value is:\n {} {}\n {} {}",
            ok_input,
            result*10f32.powf(mul), "nF",
            result*10f32.powf(mul)/1000f32, "uF",
        );
        s.pop_layer();
        s.add_layer(
            Dialog::around(TextView::new(insight))
            .button("Another", |s| {
                s.pop_layer();
                input(s);
            })
            .button("Exit", |s| s.quit())
        );
    }
}

// fn style(tui: &mut Cursive) -> Theme {
fn style(tui: &mut Cursive) {
    // let mut theme = tui.current_theme().clone();
    // tui.load_theme_file("../assets/style.toml").unwrap();
    // tui.load_toml(include_str!("../assets/style.toml")).unwrap();
    tui.set_theme(cursive::theme::Theme::default().with(|theme| {
        use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
        theme.palette[Background] = TerminalDefault;
        theme.palette[Shadow] = Dark(Yellow);
        theme.palette[View] = Dark(Green); // window background
        theme.palette[Primary] = Dark(Red); // button text and border
        theme.palette[Secondary] = Dark(Black); // user input field
        // theme.palette[Tertiary] = Dark(Black);
        theme.palette[Highlight] = Dark(Yellow); // select button
        // theme.palette[HighlightInactive] = Dark(Yellow);
        theme.palette[TitlePrimary] = Dark(Black); // yep.. title
        // theme.palette[TitleSecondary] = Light(Yellow);
    }));
    // theme
}

fn mappings(tui: &mut Cursive) {
    // exit behavior
    tui.clear_global_callbacks(cursive::event::Event::Char('q'));
    tui.set_on_pre_event(cursive::event::Event::Char('q'), |s| quit(s));
    tui.clear_global_callbacks(cursive::event::Event::CtrlChar('c'));
    tui.set_on_pre_event(cursive::event::Event::CtrlChar('c'), |s| quit(s));
    tui.clear_global_callbacks(cursive::event::Event::CtrlChar('q'));
    tui.set_on_pre_event(cursive::event::Event::CtrlChar('q'), |s| quit(s));
    tui.clear_global_callbacks(cursive::event::Event::Key(cursive::event::Key::Del));
    tui.set_on_pre_event(cursive::event::Event::Key(cursive::event::Key::Del), |s| quit(s));
    // navigation, CbSinki? hmm. Move(MoveMode::Left, Default::default())
    // tui.clear_global_callbacks(cursive::event::Event::Char('h'));
    // tui.set_on_pre_event(cursive::event::Event::Char('h'), |s| {
    // });
    // info
    tui.clear_global_callbacks(cursive::event::Event::Char('t'));
    tui.set_on_pre_event(cursive::event::Event::Char('t'), |s| info(s));
}

fn quit(tui: &mut Cursive) {
    tui.pop_layer();
    tui.add_layer(
        views::Dialog::text("Do you want to quit?")
        .button("Yes", |s| s.quit())
        .button("No", |s| {
            s.pop_layer();
            input(s);
        }),
    );
}

fn info(tui: &mut Cursive) {
    // tui.pop_layer();
    tui.add_layer(
        views::Dialog::text("
Code\tTolerance
Z   \t+80%, -20%
M   \t±20%
K   \t±10%
J   \t±5%
G   \t±2%
F   \t±1%
D   \t±0.5%
C   \t±0.25%
B   \t±0.1% 
        ")
        .title("Tolerance Marking")
        .button("Close", |s| {
            s.pop_layer();
            // input(s);
        }),
    );
}
