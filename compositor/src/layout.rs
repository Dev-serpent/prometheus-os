use crate::render::WindowHandle;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Floating,
    TilingMasterStack,
    TilingMonocle,
    TilingGrid,
    TilingEven,
}

pub struct LayoutManager {
    mode: LayoutMode,
    windows: HashMap<u64, WindowHandle>,
    focused_id: Option<u64>,
    master_count: u32,
    master_factor: f32,
    gap: u32,
    output_width: i32,
    output_height: i32,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            mode: LayoutMode::TilingMasterStack,
            windows: HashMap::new(),
            focused_id: None,
            master_count: 1,
            master_factor: 0.6,
            gap: 8,
            output_width: 1920,
            output_height: 1080,
        }
    }

    pub fn set_mode(&mut self, mode: LayoutMode) {
        self.mode = mode;
        self.arrange();
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            LayoutMode::Floating => LayoutMode::TilingMasterStack,
            LayoutMode::TilingMasterStack => LayoutMode::TilingMonocle,
            LayoutMode::TilingMonocle => LayoutMode::TilingGrid,
            LayoutMode::TilingGrid => LayoutMode::TilingEven,
            LayoutMode::TilingEven => LayoutMode::Floating,
        };
        self.arrange();
    }

    pub fn add_window(&mut self, handle: WindowHandle) {
        let id = handle.id();
        self.windows.insert(id, handle);
        self.arrange();
    }

    pub fn remove_window(&mut self, id: u64) {
        self.windows.remove(&id);
        if self.focused_id == Some(id) {
            self.focused_id = self.windows.keys().next().copied();
        }
        self.arrange();
    }

    pub fn focus_window(&mut self, id: u64) {
        if let Some(old) = self.focused_id {
            if let Some(w) = self.windows.get_mut(&old) {
                w.set_focused(false);
            }
        }
        if let Some(w) = self.windows.get_mut(&id) {
            w.set_focused(true);
        }
        self.focused_id = Some(id);
    }

    pub fn arrange(&mut self) {
        let count = self.windows.len() as u32;
        if count == 0 {
            return;
        }

        match self.mode {
            LayoutMode::Floating => {}
            LayoutMode::TilingMasterStack => self.arrange_master_stack(),
            LayoutMode::TilingMonocle => self.arrange_monocle(),
            LayoutMode::TilingGrid => self.arrange_grid(),
            LayoutMode::TilingEven => self.arrange_even(),
        }
    }

    fn arrange_master_stack(&mut self) {
        let gap = self.gap as i32;
        let count = self.windows.len() as u32;

        if count == 0 {
            return;
        }

        let window_list: Vec<u64> = self.windows.keys().copied().collect();
        let master_width = (self.output_width as f32 * self.master_factor) as i32;
        let stack_width = self.output_width - master_width - gap;

        if count == 1 {
            if let Some(w) = self.windows.get_mut(&window_list[0]) {
                w.set_size(self.output_width - gap * 2, self.output_height - gap * 2);
                w.set_position(gap, gap);
            }
            return;
        }

        let master = &window_list[0];
        let stack: &[u64] = &window_list[1..];
        let stack_count = stack.len() as i32;

        if let Some(w) = self.windows.get_mut(master) {
            w.set_size(master_width - gap, self.output_height - gap * 2);
            w.set_position(gap, gap);
        }

        let stack_height = (self.output_height - gap * (stack_count + 1)) / stack_count;
        for (i, id) in stack.iter().enumerate() {
            if let Some(w) = self.windows.get_mut(id) {
                w.set_size(stack_width - gap, stack_height);
                w.set_position(master_width + gap, gap + i as i32 * (stack_height + gap));
            }
        }
    }

    fn arrange_monocle(&mut self) {
        for w in self.windows.values_mut() {
            w.set_size(self.output_width, self.output_height);
            w.set_position(0, 0);
        }
    }

    fn arrange_grid(&mut self) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }

        let cols = (count as f32).sqrt().ceil() as i32;
        let rows = (count as f32 / cols as f32).ceil() as i32;
        let gap = self.gap as i32;

        let cell_w = (self.output_width - gap * (cols + 1)) / cols;
        let cell_h = (self.output_height - gap * (rows + 1)) / rows;

        for (i, w) in self.windows.values_mut().enumerate() {
            let col = i as i32 % cols;
            let row = i as i32 / cols;
            w.set_size(cell_w, cell_h);
            w.set_position(
                gap + col * (cell_w + gap),
                gap + row * (cell_h + gap),
            );
        }
    }

    fn arrange_even(&mut self) {
        self.arrange_grid();
    }

    pub fn update(&mut self) {}

    pub fn visible_windows(&self) -> Vec<WindowHandle> {
        self.windows.values().cloned().collect()
    }

    pub fn set_output_size(&mut self, w: i32, h: i32) {
        self.output_width = w;
        self.output_height = h;
        self.arrange();
    }

    pub fn focus_next(&mut self) {
        let ids: Vec<u64> = self.windows.keys().copied().collect();
        if ids.is_empty() {
            return;
        }
        let current = self.focused_id.unwrap_or(0);
        let pos = ids.iter().position(|&id| id == current).unwrap_or(0);
        let next = (pos + 1) % ids.len();
        self.focus_window(ids[next]);
    }

    pub fn focus_prev(&mut self) {
        let ids: Vec<u64> = self.windows.keys().copied().collect();
        if ids.is_empty() {
            return;
        }
        let current = self.focused_id.unwrap_or(0);
        let pos = ids.iter().position(|&id| id == current).unwrap_or(0);
        let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
        self.focus_window(ids[prev]);
    }
}
