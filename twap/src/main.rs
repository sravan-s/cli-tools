use std::{cmp::Ordering, error::Error, io, process::Command, time::Duration, vec};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use ratatui::{prelude::*, widgets::*};
use sysinfo::{CpuRefreshKind, Pid, RefreshKind, System};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
};

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

struct ProcessData {
    id: Pid,
    name: String,
    memory: u64,
    cpu_usage: f32,
}

struct TwapCpu {
    id: String,
    usage: u64,
}

struct App {
    state: TableState,
    items: Vec<ProcessData>,
    sys: System,
    scroll_state: ScrollbarState,
    cpus: Vec<TwapCpu>,
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
        let cpus: Vec<TwapCpu> = sys
            .cpus()
            .iter()
            .map(|c| TwapCpu {
                id: c.name().to_string(),
                usage: c.cpu_usage() as u64,
            })
            .collect();

        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((rows.len() - 1) * ITEM_HEIGHT),
            items: rows,
            cpus,
            sys,
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

    pub fn update_cpus(&mut self, cpus: Vec<TwapCpu>) {
        self.cpus = cpus;
    }
}

enum TwapInterrupt {
    TwapKeyInterrupt(Event),
    TwapCpuInterrupt(Vec<TwapCpu>),
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let (cpu_sender, mut interrupt_rx): (Sender<TwapInterrupt>, Receiver<TwapInterrupt>) =
            mpsc::channel(32);
        let key_sender = cpu_sender.clone();
        tokio::spawn(async move {
            // Wait a bit because CPU usage is based on diff.
            // Refresh CPUs again.
            let mut sys = System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            );
            let _ = sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
            sys.refresh_cpu();
            let cpus: Vec<TwapCpu> = sys
                .cpus()
                .iter()
                .map(|x| TwapCpu {
                    id: x.name().to_string(),
                    usage: x.cpu_usage() as u64,
                })
                .collect();
            cpu_sender
                .send(TwapInterrupt::TwapCpuInterrupt(cpus))
                .await
                .unwrap();
        });

        let mut event_stream = EventStream::new();
        tokio::spawn(async move {
            loop {
                let mut event_future = event_stream.next().fuse();
                let mut delay_future = Delay::new(Duration::from_millis(100)).fuse();
                select! {
                    _ = delay_future => break,
                    maybe_event = event_future => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            key_sender.send(TwapInterrupt::TwapKeyInterrupt(event)).await.unwrap();
                        }
                        _ => break
                    }
                }
                    };
            }
        });

        while let Some(message) = interrupt_rx.recv().await {
            match message {
                TwapInterrupt::TwapCpuInterrupt(cpus) => {
                    app.update_cpus(cpus);
                }
                TwapInterrupt::TwapKeyInterrupt(event) => {
                    use KeyCode::*;
                    if let Event::Key(key) = event {
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
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(12),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let top_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(rects[0]);

    render_top_chart(f, app, top_split[1]);
    render_top_panel(f, app, top_split[0]);
    render_table(f, app, rects[1]);
    render_scrollbar(f, app, rects[1]);
    render_footer(f, rects[2]);
}

fn render_top_chart(f: &mut Frame, app: &mut App, area: Rect) {
    /*
     */
    let cpu_data: Vec<(&str, u64)> = app.cpus.iter().map(|c| (c.id.as_str(), c.usage)).collect();
    let barchart = BarChart::default()
        .block(
            Block::bordered()
                .title("CPU usage")
                .title_style(Style::new().cyan()),
        )
        .label_style(Style::new().red().on_yellow())
        .bar_style(Style::new().yellow().on_red())
        .value_style(Style::new().red().bold())
        .label_style(Style::new().white())
        .data(&cpu_data)
        .bar_width(5)
        .max(100);
    f.render_widget(barchart, area);
}

fn render_top_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let lines: Vec<Line> = vec![
        format!("total_memory: {}", app.sys.total_memory()),
        // RAM and swap information:
        format!("used memory : {} bytes", app.sys.used_memory()),
        format!("total swap  : {} bytes", app.sys.total_swap()),
        format!("used swap   : {} bytes", app.sys.used_swap()),
        // Display system information:
        format!("System name:             {:?}", System::name().unwrap()),
        format!(
            "System kernel version:   {:?}",
            System::kernel_version().unwrap()
        ),
        format!(
            "System OS version:       {:?}",
            System::os_version().unwrap()
        ),
        format!(
            "System host name:        {:?}",
            System::host_name().unwrap()
        ),
        // Number of CPUs:
        format!("Number of CPUs: {}", app.sys.cpus().len()),
    ]
    .iter()
    .map(|l| Line::from(l.to_string()).style(Style::new().on_green()))
    .collect();
    f.render_widget(
        Paragraph::new(lines).block(
            Block::bordered()
                .title("System Information")
                .title_style(Style::new().on_cyan()),
        ),
        area,
    );
}

fn render_footer(f: &mut Frame, area: Rect) {
    let lines = Span::from(
        "(k)ill; Sortby: <(p/P)rocessId (n/N)ame (m/M)memory (c/C)pu>; (q)uit; (g)rep by name/pid",
    )
    .style(Style::default().add_modifier(Modifier::REVERSED))
    .style(Style::new().bold())
    .style(Style::new().fg(Color::Yellow));
    f.render_widget(Paragraph::new(lines).block(Block::bordered()), area);
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
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .block(
            Block::bordered()
                .title("Process List")
                .title_style(Style::new().on_cyan()),
        )
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let store = App::new();
    let res = run_app(&mut terminal, store).await;

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
