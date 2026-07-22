use crate::render::WindowHandle;
use std::collections::HashMap;

pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    current: usize,
    count: usize,
    animation_progress: f32,
    switching: bool,
}

struct Workspace {
    id: usize,
    name: String,
    windows: HashMap<u64, WindowHandle>,
}

impl WorkspaceManager {
    pub fn new(count: u32) -> Self {
        let count = count.max(1) as usize;
        let workspaces = (0..count)
            .map(|i| Workspace {
                id: i,
                name: if i == 0 {
                    String::from("main")
                } else {
                    format!("ws{}", i + 1)
                },
                windows: HashMap::new(),
            })
            .collect();

        Self {
            workspaces,
            current: 0,
            count,
            animation_progress: 1.0,
            switching: false,
        }
    }

    pub fn switch_to(&mut self, index: usize) {
        if index < self.count && index != self.current {
            self.current = index;
            self.switching = true;
            self.animation_progress = 0.0;
        }
    }

    pub fn next(&mut self) {
        let next = (self.current + 1) % self.count;
        self.switch_to(next);
    }

    pub fn prev(&mut self) {
        let prev = if self.current == 0 {
            self.count - 1
        } else {
            self.current - 1
        };
        self.switch_to(prev);
    }

    pub fn add_window(&mut self, handle: WindowHandle) {
        let id = handle.id();
        self.workspaces[self.current].windows.insert(id, handle);
    }

    pub fn remove_window(&mut self, id: u64) {
        for ws in &mut self.workspaces {
            ws.windows.remove(&id);
        }
    }

    pub fn move_window_to(&mut self, id: u64, workspace: usize) {
        if workspace >= self.count {
            return;
        }
        if let Some(handle) = self.workspaces[self.current].windows.remove(&id) {
            self.workspaces[workspace].windows.insert(id, handle);
        }
    }

    pub fn current_windows(&self) -> Vec<WindowHandle> {
        self.workspaces[self.current].windows.values().cloned().collect()
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn update(&mut self) {
        if self.switching {
            self.animation_progress += 0.05;
            if self.animation_progress >= 1.0 {
                self.animation_progress = 1.0;
                self.switching = false;
            }
        }
    }

    pub fn animation_progress(&self) -> f32 {
        self.animation_progress
    }

    pub fn workspace_names(&self) -> Vec<String> {
        self.workspaces.iter().map(|w| w.name.clone()).collect()
    }
}
