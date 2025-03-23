use core::panic;
use std::{fmt::Debug, fs::{self, File}, hint, io::Read};

use color_eyre::Result;
use std::io;
use std::fs::read_to_string;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
mod words;
use crate::words::Words;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = &mut ratatui::init();
    let result = App::new("../output.json".to_string()).run(terminal);
    ratatui::restore();
    result
}

// Describes application state
struct App {
    mode: Modes,
    scroll: usize,
    inputBuffer: Vec<char>,
    words: Words,
    tab: usize,
    filename: String,
}

enum Modes {
    Home,
    WordEliminator,
    SentenceBrowser,
    RuleApply,
    Save,
    Quit,
}

impl App {

    fn new(filename: String) -> App {
        App {
            mode: Modes::Home,
            scroll: 0,
            inputBuffer: vec![],
            words:     Words::new(
                read_to_string(&filename).unwrap().as_str()
            ),
            filename,
            tab: 0,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !matches!(self.mode,Modes::Quit) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self,frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            //KeyCode::Char('q') => self.mode = Modes::Quit,
            KeyCode::Esc => self.mode = Modes::Quit,
            KeyCode::Char('1') => self.change_mode(Modes::Home),
            KeyCode::Char('2') => self.change_mode(Modes::WordEliminator),
            KeyCode::Char('3') => self.change_mode(Modes::SentenceBrowser),
            KeyCode::Char('4') => self.change_mode(Modes::RuleApply),
            KeyCode::Char('5') => self.change_mode(Modes::Save),
            KeyCode::Char(' ') => {
                    self.words.require_word(self.inputBuffer.iter().collect());
                },
            KeyCode::Char(c) => {self.inputBuffer.push(c); self.scroll = 0},
            KeyCode::Backspace => {self.inputBuffer.pop(); self.scroll = 0},
            KeyCode::Enter => {
                    match self.mode {
                        Modes::RuleApply => self.apply_rules(self.scroll),
                        _ => self.words.remove_words(vec![self.inputBuffer.iter().collect()]),
                    }
                },
            KeyCode::Tab => self.tab = self.tab.wrapping_add(1),
            KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
            KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
            _ => {}
        }
    }

    fn change_mode(&mut self, new_mode : Modes) {
        self.inputBuffer.clear();
        self.tab = 0;
        self.scroll = 0;
        self.mode = new_mode;
    }

    fn input_buffer_display(&self) -> String {
        format!("{: <6}", self.inputBuffer.iter().collect::<String>())
    }

    fn apply_rules(&mut self, index: usize) {
        match index {
            0 => todo!(),
            1 => {
                //gets rid of two letter words that no one uses
                self.words.remove_words(
                ["aa","ab","ac","ae","af","ag","ai","aj","ak","al","ap","ar","au","av","az","ba","bb","bc","bd","bg","bk","bl","bm","bo","bp","br","bs","bt","bw","ca","cb","cc","cd","ce","cf","cg","ch","ci","cj","cl","cm","cn","co","cp","cr","cs","ct","cu","cv","cw","cz","db","dc","dd","de","df","dg","dh","di","dj","dk","dl","dm","dp","dr","ds","dt","du","dv","dx","ea","ec","ed","ee","ef","eg","el","em","en","ep","eq","er","es","eu","ev","ez","fc","fd","fe","ff","fg","fi","fl","fm","fo","fp","fr","fs","ft","fu","fw","fx","fy","ga","gb","gc","gd","ge","gg","gi","gl","gm","gp","gr","gs","gt","hb","hc","hd","hh","hk","hl","ho","hp","hq","hr","hs","ht","hu","hz","ia","ic","id","ie","ii","il","io","ip","ir","iv","ix","ja","jc","jd","je","jj","jm","jo","jp","jr","js","ka","kb","kg","km","ko","ks","kw","ky","la","lb","lc","ld","le","lf","lg","li","ll","lm","ln","lo","lp","ls","lt","lu","ma","mb","mc","md","mf","mg","mh","mi","mj","ml","mm","mn","mo","mp","mt","mu","mv","mw","mx","na","nb","nd","ne","ng","nh","ni","nl","nm","nn","np","nr","ns","nt","nu","nv","nw","nz","ob","oc","oe","og","om","oo","op","os","ot","ou","oz","pa","pb","pc","pd","pe","pf","pg","ph","pj","pk","pl","pm","pn","po","pp","pr","ps","pt","qc","qt","ra","rb","rc","rd","re","rf","rg","rh","ri","rj","rl","rm","rn","ro","rp","rr","rs","rt","ru","rv","rw","rx","sa","sb","sc","sd","se","sf","sg","sh","si","sk","sl","sm","sn","sp","sq","sr","ss","st","su","sv","sw","ta","tb","tc","td","te","tf","th","ti","tm","tn","tp","tr","ts","tt","tu","tv","tx","ty","uc","ui","uk","ul","um","un","ut","uv","uw","va","vb","vc","ve","vg","vi","vp","vs","vt","wa","wb","wc","wi","wm","wn","wp","wr","ws","wt","wu","wv","ww","wx","wy","xi","xl","xp","xx","ye","yn","yr","yu","za","zu"]
                .into_iter()
                .map(|word| word.to_string())
                .collect()                
                );
            },
            _ => return,
        }
    }
}


impl Widget for &App {

    fn render(self, area: Rect, buf: &mut Buffer) {
        //top bar: Lists words, transforms remaining
        let title = Line::from(vec![
            Span::from(" The Eliminator ").bold(),
            Span::from(format!("[Combinations: {} | Transforms: {}]",
                self.words.total_combinations(),
                self.words.total_transforms())
            )
        ]);

        //main area: interface for the current mode

            //Home
        let (random_transform,random_words) = self.words.random_solution();
        let home = Text::from(vec![
            Line::from("Possibility List Loaded"),
            Line::from(format!("Total Words: {}",self.words.total_words())),
            Line::from(format!("Total Possible Transforms: {}", self.words.total_transforms())),
            Line::from(format!("Total Possible Combinations: {}", self.words.total_combinations())),
            Line::from(format!("A random solution is below, press <Tab> to generate more:")),
            Line::from(random_transform).centered(),
            Line::from(random_words).centered(),
        ]);

            //Word Browser
        let word_browser = {
            
            enum WordMode {
                Popular,
                Critical,
            }
            let current_word_mode = match self.tab % 2 {
                1 => WordMode::Critical,
                _ => WordMode::Popular,
            };
            
            let mut lines = vec![
                Line::from(vec![
                    Span::from("Type a word to search, "),
                    Span::from("<Enter>").blue(),
                    Span::from(" to delete it or "),
                    Span::from("<Space>").blue(),
                    Span::from(" to require it."),
                ]).italic().centered(),
                Line::from(self.input_buffer_display()).centered().underlined(),
                Line::from(vec![
                    match current_word_mode {
                        WordMode::Popular => Span::from("Most Combinations").bold(),
                        WordMode::Critical => Span::from("Critical Words").bold(),
                    },
                    Span::from(" (Tab to switch)")
                ])                   
            ];

            match current_word_mode {
                WordMode::Popular => {
                    let selected_words = self.words.get_top(self.scroll..(self.scroll + 80),
                    |(word,_count)| {
                                self.inputBuffer.len() < 1 || word.starts_with(&self.inputBuffer.iter().collect::<String>())
                    });

                    selected_words.into_iter()
                        .map(|(word, count)| 
                            Line::from(format!("  {: <8}: {: >18}",word,count))
                        )
                        .for_each(|line| lines.push(line));
                },
                WordMode::Critical => {
                    self.words.critical_words()
                        .into_iter()
                        .filter(|(word, _count)| word.starts_with(&self.inputBuffer.iter().collect::<String>()))
                        .skip(self.scroll)
                        .for_each(|(word,count)| lines.push(
                            Line::from(format!("  {: <8}: {: >18}",word,count))
                        ));
                }

            }

            Text::from(lines)
        };

            //Rules Applyer
        let rules_apply = {
            let mut lines = vec![
                Line::from("Rules to Apply").centered().bold(),
                Line::from(vec![
                    Span::from("Select a rule and hit "),
                    Span::from("<Enter>").blue(),
                    Span::from(" to apply it. This is not reversible."),
                ]).centered().italic().centered(),
                Line::from(""),
            ];

            let bullet = |target: usize| {
                if self.scroll == target {
                    Span::from(String::from(" > ")).bold()
                } else {
                    Span::from(String::from(" - "))
                }
            };

            let create_line = |position,text: &str| {
                Line::from(vec![bullet(position),Span::from(text.to_string())])
            };

            // Letters from the left of the original transform on the card must all be used within the blank spaces
            lines.push(create_line(0, "All letters from L side of original xform must be used"));

            // Remove unlikely two-letter words
            lines.push(create_line(1, "Eliminate ridiculous two letter words like 'oo'"));

            
            Text::from(lines)
        };
            

        //text hints bar: gives text hints
        let hint_bar = Line::from(self.inputBuffer.iter().collect::<String>());
        //bottom bar: lists modes and indicates current mode


        let bottom_bar = Line::from(vec![
            "|".into(),
            if matches!(self.mode,Modes::Home) {" Home ".on_light_magenta()} else {" Home ".into()},
            "<1> |".blue().bold(),
            if matches!(self.mode,Modes::WordEliminator) {" Word List ".on_light_magenta()} else {" Word List ".into()},
            "<2> |".blue().bold(),
            if matches!(self.mode,Modes::SentenceBrowser) {" Sentence Browser ".on_light_magenta()} else {" Sentence Browser ".into()},
            "<3> |".blue().bold(),
            if matches!(self.mode,Modes::RuleApply) {" Apply Rules ".on_light_magenta()} else {" Apply Rules ".into()},
            "<4> |".blue().bold(),
            if matches!(self.mode,Modes::Save) {" Save ".on_light_magenta()} else {" Save ".into()},
            "<5> |".blue().bold(),
            if matches!(self.mode,Modes::Quit) {" Quit ".on_light_magenta()} else {" Quit ".into()},
            "<Esc> ".blue().bold(),
            "|".into(),
        ]);

        let block = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(bottom_bar.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.scroll.to_string().yellow(),
        ]),
        hint_bar]);

        Paragraph::new(
            match self.mode {
                Modes::Home => home,
                Modes::WordEliminator => word_browser,
                Modes::RuleApply => rules_apply,
                _ => counter_text,
            }
        )
            .left_aligned()
            .block(block)
            .render(area, buf);
    }

}

/*
pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(frame, app, chunks[1]),
        1 => draw_second_tab(frame, app, chunks[1]),
        2 => draw_third_tab(frame, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ])
    .split(area);
    draw_gauges(frame, app, chunks[0]);
    draw_charts(frame, app, chunks[1]);
    draw_text(frame, chunks[2]);
}
     */
