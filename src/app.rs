use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packages,
    Search,
    Updates,
}

impl Tab {
    pub fn next(self) -> Self {
        match self {
            Tab::Packages => Tab::Search,
            Tab::Search => Tab::Updates,
            Tab::Updates => Tab::Packages,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Tab::Packages => Tab::Updates,
            Tab::Search => Tab::Packages,
            Tab::Updates => Tab::Search,
        }
    }

    pub fn titles() -> &'static [&'static str] {
        &["Packages", "Search", "Updates"]
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Packages => 0,
            Tab::Search => 1,
            Tab::Updates => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
}

pub struct App {
    pub current_tab: Tab,
    pub should_quit: bool,
    pub selected_index: usize,
    pub packages: Vec<Package>,
    pub load_error: Option<String>,
    pub show_details: bool,
    pub package_details: Option<Vec<String>>,
    pub searching: bool,
    pub search_query: String,
}

impl App {
    pub fn new() -> Self {
        let (packages, load_error) = match load_installed_packages() {
            Ok(pkgs) => (pkgs, None),
            Err(e) => (Vec::new(), Some(e)),
        };

        Self {
            current_tab: Tab::Packages,
            should_quit: false,
            selected_index: 0,
            packages,
            load_error,
            show_details: false,
            package_details: None,
            searching: false,
            search_query: String::new(),
        }
    }

    pub fn next_item(&mut self) {
        let len = self.list_len();
        if len > 0 {
            self.selected_index = (self.selected_index + 1) % len;
        }
    }

    pub fn prev_item(&mut self) {
        let len = self.list_len();
        if len > 0 && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn first_item(&mut self) {
        self.selected_index = 0;
    }

    pub fn last_item(&mut self) {
        let len = self.list_len();
        if len > 0 {
            self.selected_index = len - 1;
        }
    }

    pub fn visible_packages(&self) -> Vec<&Package> {
        if self.search_query.is_empty() {
            self.packages.iter().collect()
        } else {
            let q = self.search_query.to_lowercase();
            self.packages
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&q))
                .collect()
        }
    }

    pub fn open_search(&mut self) {
        self.close_details();
        self.search_query.clear();
        self.selected_index = 0;
        self.searching = true;
    }

    pub fn close_search(&mut self) {
        self.searching = false;
        self.search_query.clear();
        self.selected_index = 0;
    }

    pub fn confirm_search(&mut self) {
        self.searching = false;
    }

    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.selected_index = 0;
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.selected_index = 0;
    }

    pub fn toggle_details(&mut self) {
        if self.show_details {
            self.show_details = false;
            self.package_details = None;
        } else {
            self.open_details();
        }
    }

    pub fn open_details(&mut self) {
        if self.current_tab != Tab::Packages {
            return;
        }
        let name = {
            let visible = self.visible_packages();
            if visible.is_empty() {
                return;
            }
            visible[self.selected_index].name.clone()
        };
        self.package_details = Some(match load_package_details(&name) {
            Ok(lines) => lines,
            Err(e) => vec![format!("Error: {e}")],
        });
        self.show_details = true;
    }

    pub fn close_details(&mut self) {
        self.show_details = false;
        self.package_details = None;
    }

    fn list_len(&self) -> usize {
        match self.current_tab {
            Tab::Packages => self.visible_packages().len(),
            _ => 0,
        }
    }
}

// loads in this format::
// example:

//spotify 1:1.2.77.358-1
// sqlite 3.53.2-1
// squeekboard 1.43.1-5
// sratom 0.6.22-1
// srt 1.5.5-1
// starship 1.25.1-1
// startup-notification 0.12-9
// steam 1.0.0.85-7
// steam-devices 1.0.0.85-7
// stremio 4.4.183-1
// stremio-debug 4.4.183-1

fn load_installed_packages() -> Result<Vec<Package>, String> {
    let output = Command::new("pacman")
        .arg("-Q")
        .output()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("pacman -Q failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ' ');
            let name = parts.next()?.to_string();
            let version = parts.next()?.to_string();
            Some(Package { name, version })
        })
        .collect();

    Ok(packages)
}

fn load_package_details(name: &str) -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(["-Qi", name])
        .output()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("pacman -Qi failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|l| l.to_string()).collect())
}
