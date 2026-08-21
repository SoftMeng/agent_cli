use std::collections::VecDeque;

const MAX_HISTORY: usize = 100;

// ============ Input Buffer ============

pub struct InputBuffer {
    pub text: String,
    pub cursor: usize,
    history: VecDeque<String>,
    history_idx: Option<usize>,
    draft: String,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            history: VecDeque::new(),
            history_idx: None,
            draft: String::new(),
        }
    }

    fn len_chars(&self) -> usize {
        self.text.chars().count()
    }

    pub fn cursor_chars(&self) -> usize {
        self.cursor
    }

    pub fn insert_char(&mut self, c: char) {
        let byte_idx = self
            .text
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());
        self.text.insert(byte_idx, c);
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let prev = self.cursor - 1;
        let start = self.text.char_indices().nth(prev).map(|(i, _)| i).unwrap();
        let end = self
            .text
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());
        self.text.replace_range(start..end, "");
        self.cursor = prev;
    }

    pub fn delete_forward(&mut self) {
        let len = self.len_chars();
        if self.cursor >= len {
            return;
        }
        let start = self
            .text
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap();
        let end = self
            .text
            .char_indices()
            .nth(self.cursor + 1)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());
        self.text.replace_range(start..end, "");
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.len_chars() {
            self.cursor += 1;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.len_chars();
    }

    fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.history_idx = None;
    }

    pub fn recall_history(&mut self, direction: i32) {
        if self.history.is_empty() {
            return;
        }
        let idx = match self.history_idx {
            None => {
                self.draft = self.text.clone();
                if direction > 0 {
                    return;
                }
                self.history.len() - 1
            }
            Some(i) => {
                if direction > 0 {
                    if i + 1 >= self.history.len() {
                        self.text = self.draft.clone();
                        self.cursor = self.text.chars().count();
                        self.history_idx = None;
                        return;
                    }
                    i + 1
                } else if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
        };
        let entry = self.history[idx].clone();
        self.text = entry;
        self.cursor = self.text.chars().count();
        self.history_idx = Some(idx);
    }

    pub fn commit(&mut self) -> String {
        let committed = self.text.clone();
        if !committed.trim().is_empty()
            && self.history.back().map(|s| s != &committed).unwrap_or(true)
        {
            if self.history.len() >= MAX_HISTORY {
                self.history.pop_front();
            }
            self.history.push_back(committed.clone());
        }
        self.clear();
        committed
    }
}
