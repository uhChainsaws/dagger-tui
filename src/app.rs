use std::{error, fmt::Display};

use sysinfo::{System, SystemExt};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const TRAIL_LEN: usize = 140;

#[derive(Debug)]
pub enum DaggerTab {
    Processes,
    RAM,
}

impl Display for DaggerTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DaggerTab::Processes => {"procs"},
            DaggerTab::RAM => {"ramem"}
        };
        write!(f, "{s}")
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// number of processes in time
    pub trail: Box<[u64]>,
    /// increments every tick and overflows to 0
    pub ticker: usize,
    /// current tab
    pub tab: DaggerTab,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            trail: Box::new([0; TRAIL_LEN]),
            ticker: 0,
            tab: DaggerTab::Processes,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.ticker += 1;
        if self.ticker >= TRAIL_LEN-1 {
            self.ticker = 0
        }
        match self.tab {
            DaggerTab::Processes => {self.update_processes_counter()},
            DaggerTab::RAM => {self.update_ram_counter()}
        }
    }

    pub fn update_processes_counter(&mut self) {
        let mut system = System::new();
        system.refresh_processes();
        self.trail[self.ticker % TRAIL_LEN] = system.processes().len() as u64;        
    }

    pub fn update_ram_counter(&mut self) {
        let mut system = System::new();
        system.refresh_memory();
        self.trail[self.ticker % TRAIL_LEN] = system.used_memory();        
    }

    pub fn get_all_memory(&mut self) -> u64 {
        let mut system = System::new();
        system.refresh_memory();
        system.total_memory()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Cycles tab forward
    pub fn cycle_tab_forward(&mut self) {
        self.trail.fill(0);
        self.tab = match self.tab {
            DaggerTab::Processes => {DaggerTab::RAM},
            DaggerTab::RAM => {DaggerTab::Processes},
        };
    }

    /// Cycles tab backward
    pub fn cycle_tab_backward(&mut self) {
        self.trail.fill(0);
        self.tab = match self.tab {
            DaggerTab::RAM => {DaggerTab::Processes},
            DaggerTab::Processes => {DaggerTab::RAM},
        };
    }
}
