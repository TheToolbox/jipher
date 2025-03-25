use core::time;
use std::vec;
use color_eyre::Result;
use std::fs::read_to_string;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
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
    scroll_level: usize,
    input_buffer: Vec<char>,
    words: Words,
    tab: usize,
    #[allow(dead_code)]
    file_name: String,
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
        println!("Loading file {}...", filename);
        App {
            mode: Modes::Home,
            scroll_level: 0,
            input_buffer: vec![],
            words:     Words::new(
                read_to_string(&filename).unwrap().as_str()
            ),
            file_name: filename,
            tab: 0,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        println!("Starting terminal application...");
        std::thread::sleep(time::Duration::from_secs(1));
        terminal.clear()?;
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

    fn scroll(&mut self, x: isize) {
        self.scroll_level = self.scroll_level.saturating_add_signed(x);
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
                    match self.mode {
                        Modes::SentenceBrowser => {
                            let to_require = self.input_buffer.iter().collect();
                            self.words.require_word_positional(to_require, self.tab);
                        },
                        _ => {
                            let to_require = self.input_buffer.iter().collect();
                            self.words.require_word(to_require);
                        }
                    }
                },
            KeyCode::Char(c) => {self.input_buffer.push(c); self.scroll_level = 0},
            KeyCode::Backspace => {self.input_buffer.pop(); self.scroll_level = 0},
            KeyCode::Enter => {
                    match self.mode {
                        Modes::RuleApply => self.apply_rules(self.scroll_level),
                        Modes::SentenceBrowser => {
                            let to_remove = self.input_buffer.iter().collect();
                            self.words.remove_words_positional(vec![to_remove],self.tab);
                        }
                        _ => 
                            {
                                let to_remove = self.input_buffer.iter().collect();
                                self.words.remove_words(vec![to_remove]);
                            },
                    }
                },
            KeyCode::Tab => self.tab = 
                if matches!(self.mode,Modes::SentenceBrowser) && self.tab >= self.words.sentence_length() - 1 {
                    0
                } else {
                    self.tab.wrapping_add(1)
                }
            ,
            KeyCode::Down => self.scroll(1),
            KeyCode::Up => self.scroll(-1),
            KeyCode::PageDown => self.scroll(10),
            KeyCode::PageUp => self.scroll(-10),
            _ => {}
        }
    }

    fn change_mode(&mut self, new_mode : Modes) {
        self.input_buffer.clear();
        self.tab = 0;
        self.scroll_level = 0;
        self.mode = new_mode;
    }

    fn input_buffer_display(&self) -> String {
        format!("{: <6}", self.input_buffer.iter().collect::<String>())
    }

    fn apply_rules(&mut self, index: usize) {
        match index {
            0 => {
                //gather complete word lists for each position (can be cribbed from positional hists)
                //for each word in a given position
                    //check blank spaces
                //okay this rapidly becomes extremely complicated (infeasible? because each word will develop dependencies on other words. very difficult to calculate much less present)
                todo!()
            },
            1 => {
                //gets rid of two letter words that no one uses
                self.words.remove_words(
                ["aa","ab","ac","ae","af","ag","ai","aj","ak","al","ap","ar","au","av","az","ba","bb","bc","bd","bg","bk","bl","bm","bo","bp","br","bs","bt","bw","ca","cb","cc","cd","ce","cf","cg","ch","ci","cj","cl","cm","cn","co","cp","cr","cs","ct","cu","cv","cw","cz","db","dc","dd","de","df","dg","dh","di","dj","dk","dl","dm","dp","dr","ds","dt","du","dv","dx","ea","ec","ed","ee","ef","eg","el","em","en","ep","eq","er","es","eu","ev","ez","fc","fd","fe","ff","fg","fi","fl","fm","fo","fp","fr","fs","ft","fu","fw","fx","fy","ga","gb","gc","gd","ge","gg","gi","gl","gm","gp","gr","gs","gt","hb","hc","hd","hh","hk","hl","ho","hp","hq","hr","hs","ht","hu","hz","ia","ic","id","ie","ii","il","io","ip","ir","iv","ix","ja","jc","jd","je","jj","jm","jo","jp","jr","js","ka","kb","kg","km","ko","ks","kw","ky","la","lb","lc","ld","le","lf","lg","li","ll","lm","ln","lo","lp","ls","lt","lu","ma","mb","mc","md","mf","mg","mh","mi","mj","ml","mm","mn","mo","mp","mt","mu","mv","mw","mx","na","nb","nd","ne","ng","nh","ni","nl","nm","nn","np","nr","ns","nt","nu","nv","nw","nz","ob","oc","oe","og","om","oo","op","os","ot","ou","oz","pa","pb","pc","pd","pe","pf","pg","ph","pj","pk","pl","pm","pn","po","pp","pr","ps","pt","qc","qt","ra","rb","rc","rd","re","rf","rg","rh","ri","rj","rl","rm","rn","ro","rp","rr","rs","rt","ru","rv","rw","rx","sa","sb","sc","sd","se","sf","sg","sh","si","sk","sl","sm","sn","sp","sq","sr","ss","st","su","sv","sw","ta","tb","tc","td","te","tf","th","ti","tm","tn","tp","tr","ts","tt","tu","tv","tx","ty","uc","ui","uk","ul","um","un","ut","uv","uw","va","vb","vc","ve","vg","vi","vp","vs","vt","wa","wb","wc","wi","wm","wn","wp","wr","ws","wt","wu","wv","ww","wx","wy","xi","xl","xp","xx","ye","yn","yr","yu","za","zu"]
                .into_iter()
                .map(|word| word.to_string())
                .collect()                
                );
            },
            2 => {
                //removes a few common words that eliminate a large swath of transforms
                self.words.remove_words([
                    "lucy",
                    "nfl",
                    "rico",
                    "dana",
                    "nsw",
                    "toyota",
                    "nfl",
                    "nba",
                    "orgy",
                    "nbc",
                    "asthma",
                    "sweden",
                    "apollo",
                    "sql",
                    "acid",
                    "ipod",
                    "incl",
                    "divx",
                    "ciao",
                ].into_iter().map(|word| word.to_string()).collect());
            }
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
                    let selected_words = self.words.get_top(self.scroll_level..(self.scroll_level + 80),
                    |(word,_count)| {
                                self.input_buffer.len() < 1 || word.starts_with(&self.input_buffer.iter().collect::<String>())
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
                        .filter(|(word, _count)| word.starts_with(&self.input_buffer.iter().collect::<String>()))
                        .skip(self.scroll_level)
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
                if self.scroll_level == target {
                    Span::from(String::from(" > ")).bold()
                } else {
                    Span::from(String::from(" - "))
                }
            };

            let create_line = |position,text: &str| {
                Line::from(vec![bullet(position),Span::from(text.to_string())])
            };

            // Letters from the left of the original transform on the card must all be used within the blank spaces
            lines.push(create_line(0, "All letters from L side of original xform must be used within blanks"));

            // Remove unlikely two-letter words
            lines.push(create_line(1, "Eliminate ridiculous two letter words like 'oo'"));

            lines.push(create_line( 2, "Eliminate several unlikely words (lucy, nsw, toyota) that greatly reduce state-space"));

            Text::from(lines)
        };

            //sentence browser
        let sentence_browser = {
            let puzzle = "t. .i.d t..rt. .o .o. t... na. ..ne y.. .w.lm. ..cy d.ne".to_string();
            
            let words = puzzle.split(" ")
                .map(|word| " ".repeat(word.len()))
                .collect::<Vec<_>>();
            let words2 = words.clone();

            let word_input = words.into_iter()
                .enumerate()
                .flat_map(|(index, word)| [
                    if self.tab == index {
                        let buffer : String = self.input_buffer.iter().collect();
                        Span::from(
                            format!("{: <width$}", buffer, width = word.len())
                        ).underlined().blue()
                    } else {
                        Span::from(word).underlined()
                    },
                    Span::from(" ")
                ])
                .collect::<Vec<Span>>();

            let mut popular_words = (self.scroll_level..self.scroll_level+40).into_iter()
                .map(|line_index| {
                    Line::from(
                        self.words.positional_histograms().into_iter()
                        .enumerate()
                        .map(|(word_index, word_hist)| {
                            if let Some((word,_)) = word_hist.most_popular()
                                .iter()
                                .filter(|(word,_)| {
                                    word_index != self.tab || word.starts_with(&self.input_buffer.iter().collect::<String>())
                                })
                                .nth(line_index)
                                {
                                    word.clone() + " "
                                } else {
                                    words2[word_index].clone() + " "
                                }
                        })
                        .collect::<String>()
                    )
                })
                .collect::<Vec<Line>>();

            let mut lines = vec![
                Line::from(vec![
                    Span::from("Use "),
                    Span::from("<Tab>").blue(),
                    Span::from(" to switch words. "),
                ]),
                Line::from(vec![
                    Span::from("Use "),
                    Span::from("<Enter>").blue(),
                    Span::from(" to eliminate words or "),
                    Span::from("<Space>").blue(),
                    Span::from(" to lock in words at this position. "),
                ]),
                Line::from(""),
                Line::from(puzzle).dim(),                   
                Line::from(
                    word_input
                ),
            ];

            lines.append(&mut popular_words);

            Text::from(lines)
        };
            

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

        let outer_block = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(bottom_bar.centered())
            .border_set(border::ROUNDED)
            .padding(ratatui::widgets::Padding::new(2, 2, 0, 0));
        
        let paragraph = Paragraph::new(
            match self.mode {
                Modes::Home => home,
                Modes::WordEliminator => word_browser,
                Modes::RuleApply => rules_apply,
                Modes::SentenceBrowser => sentence_browser,
                _ => Text::from("Not yet implemented..."),
            }
        )
            .left_aligned()
            .block(outer_block);


            let [main_area, elim_area] = Layout::horizontal([Constraint::Percentage(100),Constraint::Min(20)])
                .areas(area);

        paragraph.render(main_area, buf);

        let elimblock = Block::bordered()
        .title("History")
        .border_set(border::ROUNDED);

        let elim_paragraph = Paragraph::new(
                Text::from(
                    self.words.history()
                )
            )
            .left_aligned()
            .block(elimblock);

        elim_paragraph.render(elim_area, buf);
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
