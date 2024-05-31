use std::{cmp::Ordering, error::Error, io, process::Command};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use sysinfo::{Pid, System};

struct ProcessData {
    id: Pid,
    name: String,
    memory: u64,
    cpu_usage: f32,
}

struct App {
    state: TableState,
    items: Vec<ProcessData>,
    scroll_state: ScrollbarState,
}

const ITEM_HEIGHT: usize = 4;

impl App {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let rows: Vec<ProcessData> = sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessData {
                id: *pid,
                name: process.name().to_string(),
                memory: process.memory(),
                cpu_usage: process.cpu_usage(),
            })
            .collect();
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((rows.len() - 1) * ITEM_HEIGHT),
            items: rows,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn sort_by(&mut self, sort_by: SortBy) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut rows: Vec<ProcessData> = sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessData {
                id: *pid,
                name: process.name().to_string(),
                memory: process.memory(),
                cpu_usage: process.cpu_usage(),
            })
            .collect();
        match sort_by {
            SortBy::CpuAsc => rows.sort_by(|r1, r2| {
                r1.cpu_usage
                    .partial_cmp(&r2.cpu_usage)
                    .unwrap_or(Ordering::Equal)
            }),
            SortBy::CpuDesc => rows.sort_by(|r1, r2| {
                r2.cpu_usage
                    .partial_cmp(&r1.cpu_usage)
                    .unwrap_or(Ordering::Equal)
            }),
            SortBy::MemoryAsc => rows.sort_by(|r1, r2| r1.memory.cmp(&r2.memory)),
            SortBy::MemoryDesc => rows.sort_by(|r1, r2| r2.memory.cmp(&r1.memory)),
            SortBy::NameAsc => rows.sort_by(|r1, r2| r1.name.cmp(&r2.name)),
            SortBy::NameDesc => rows.sort_by(|r1, r2| r2.name.cmp(&r1.name)),
            SortBy::PidAsc => rows.sort_by(|r1, r2| r1.id.cmp(&r2.id)),
            SortBy::PidDesc => rows.sort_by(|r1, r2| r2.id.cmp(&r1.id)),
        }
        self.items = rows;
    }

    pub fn kill(&mut self) {
        let selected_idex = match self.state.selected() {
            Some(i) => i,
            _ => {
                return;
            }
        };
        let process_to_kill = match self.items.get(selected_idex) {
            Some(p) => p.id,
            _ => {
                return;
            }
        };
        let _output = Command::new("kill")
            .arg("-9")
            .arg(process_to_kill.to_string())
            .output()
            .expect("Failed to execute kill command");
        self.items.remove(selected_idex);
    }
}

enum SortBy {
    CpuAsc,
    CpuDesc,
    MemoryAsc,
    MemoryDesc,
    NameAsc,
    NameDesc,
    PidAsc,
    PidDesc,
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Down => app.next(),
                    Up => app.previous(),
                    Char('c') => app.sort_by(SortBy::CpuAsc),
                    Char('C') => app.sort_by(SortBy::CpuDesc),
                    Char('m') => app.sort_by(SortBy::MemoryAsc),
                    Char('M') => app.sort_by(SortBy::MemoryDesc),
                    Char('n') => app.sort_by(SortBy::NameAsc),
                    Char('N') => app.sort_by(SortBy::NameDesc),
                    Char('p') => app.sort_by(SortBy::PidAsc),
                    Char('P') => app.sort_by(SortBy::PidDesc),
                    Char('k') | Char('K') => app.kill(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    render_table(f, app, rects[0]);

    render_scrollbar(f, app, rects[0]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let rows = app.items.iter().map(|i| {
        Row::new(vec![
            i.id.to_string(),
            i.name.to_string(),
            i.memory.to_string(),
            i.cpu_usage.to_string(),
        ])
    });
    let header = Row::new(vec!["process_id", "name", "memory_usage", "cpu_usage"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let footer = Row::new(vec![
        "(k)ill",
        "Sort: (c/C)pu, (m/M), (n/N), (p/P)",
        "(g)rep by name/pid",
    ])
    .style(Style::new().bold())
    .top_margin(1);

    let widths = [
        Constraint::Length(20),
        Constraint::Length(40),
        Constraint::Length(20),
        Constraint::Length(20),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(header)
        .footer(footer)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>");

    f.render_stateful_widget(table, area, &mut app.state);
}

fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    /*
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let store = App::new();
    let res = run_app(&mut terminal, store);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
