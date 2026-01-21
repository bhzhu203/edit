use std::collections::HashSet;
use std::cmp::Ordering;

use crate::buffer::{TextBuffer, CursorMovement};
use crate::helpers::Point;

/// Represents a single auto-completion suggestion
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionItem {
    /// The text to insert when this completion is selected
    pub label: String,
    /// Optional detail/description for the completion
    pub detail: Option<String>,
    /// Sort text (defaults to label if not provided)
    pub sort_text: Option<String>,
}

impl CompletionItem {
    pub fn new(label: String) -> Self {
        CompletionItem {
            label,
            detail: None,
            sort_text: None,
        }
    }

    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn with_sort_text(mut self, sort_text: String) -> Self {
        self.sort_text = Some(sort_text);
        self
    }
}

/// State for auto-completion functionality
#[derive(Debug, Clone)]
pub struct AutoCompletionState {
    /// Current prefix being completed
    pub prefix: String,
    /// Current position where completion started
    pub start_pos: Point,
    /// Current position where completion ends
    pub end_pos: Point,
    /// Available completion items
    pub items: Vec<CompletionItem>,
    /// Current selected index in the items list
    pub selected_index: usize,
    /// Whether completion is currently active
    pub is_active: bool,
    /// Whether to show the completion popup
    pub show_popup: bool,
}

impl AutoCompletionState {
    pub fn new() -> Self {
        AutoCompletionState {
            prefix: String::new(),
            start_pos: Point::default(),
            end_pos: Point::default(),
            items: Vec::new(),
            selected_index: 0,
            is_active: false,
            show_popup: false,
        }
    }

    pub fn reset(&mut self) {
        self.prefix.clear();
        self.items.clear();
        self.selected_index = 0;
        self.is_active = false;
        self.show_popup = false;
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn accept_current(&mut self, buffer: &mut TextBuffer) -> bool {
        if self.is_active && !self.items.is_empty() && self.selected_index < self.items.len() {
            let item = &self.items[self.selected_index];
            
            // Delete the current prefix
            buffer.delete(CursorMovement::Grapheme, -(self.prefix.len() as isize));
            
            // Insert the completion
            buffer.write_canon(item.label.as_bytes());
            
            self.reset();
            true
        } else {
            false
        }
    }
}

impl Default for AutoCompletionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto-completion provider trait
pub trait CompletionProvider {
    fn get_completions(&self, buffer: &TextBuffer, prefix: &str) -> Vec<CompletionItem>;
}

/// Basic word-based completion provider that extracts words from the current buffer
pub struct WordCompletionProvider;

impl CompletionProvider for WordCompletionProvider {
    fn get_completions(&self, buffer: &TextBuffer, prefix: &str) -> Vec<CompletionItem> {
        if prefix.is_empty() {
            return Vec::new();
        }

        let mut words = HashSet::new();
        
        // Extract words from the buffer
        let mut current_offset = 0;
        let buffer_len = buffer.text_length();
        
        while current_offset < buffer_len {
            let chunk = buffer.read_forward(current_offset);
            if chunk.is_empty() {
                break;
            }

            // Find words in the chunk
            let mut word_start = None;
            for (i, &byte) in chunk.iter().enumerate() {
                let ch = byte as char;
                
                if ch.is_alphanumeric() || ch == '_' {
                    if word_start.is_none() {
                        word_start = Some(i);
                    }
                } else {
                    if let Some(start) = word_start {
                        let word_bytes = &chunk[start..i];
                        if let Ok(word) = std::str::from_utf8(word_bytes) {
                            if word.len() >= prefix.len() && word.starts_with(prefix) && word != prefix {
                                words.insert(word.to_string());
                            }
                        }
                        word_start = None;
                    }
                }
            }
            
            // Handle word at the end of chunk
            if let Some(start) = word_start {
                let word_bytes = &chunk[start..];
                if let Ok(word) = std::str::from_utf8(word_bytes) {
                    if word.len() >= prefix.len() && word.starts_with(prefix) && word != prefix {
                        words.insert(word.to_string());
                    }
                }
            }
            
            current_offset += chunk.len();
        }

        // Convert to CompletionItems and sort
        let mut completions: Vec<_> = words
            .into_iter()
            .map(|word| CompletionItem::new(word))
            .collect();

        // Sort completions by relevance (length, alphabetical)
        completions.sort_by(|a, b| {
            // Prioritize shorter completions
            match a.label.len().cmp(&b.label.len()) {
                Ordering::Equal => a.label.cmp(&b.label),
                other => other,
            }
        });

        completions
    }
}

/// Main auto-completion controller
pub struct AutoCompleter {
    pub state: AutoCompletionState,
    pub provider: Box<dyn CompletionProvider>,
}

impl AutoCompleter {
    pub fn new(provider: Box<dyn CompletionProvider>) -> Self {
        AutoCompleter {
            state: AutoCompletionState::new(),
            provider,
        }
    }

    pub fn trigger_completion(&mut self, buffer: &TextBuffer) {
        // Get the current word being typed
        let current_pos = buffer.cursor_logical_pos();
        let current_offset = buffer.cursor.offset;
        
        // Extract the prefix (word being typed)
        let prefix = self.extract_prefix(buffer, current_offset);
        
        if prefix.len() >= 2 { // Only show completions for prefixes of 2+ characters
            let completions = self.provider.get_completions(buffer, &prefix);
            
            if !completions.is_empty() {
                self.state.prefix = prefix;
                self.state.items = completions;
                self.state.selected_index = 0;
                self.state.is_active = true;
                self.state.show_popup = true;
                
                // Calculate start position (position before the prefix)
                let start_offset = current_offset - self.state.prefix.len();
                self.state.start_pos = self.offset_to_point(buffer, start_offset);
                self.state.end_pos = current_pos;
            } else {
                self.state.reset();
            }
        } else {
            self.state.reset();
        }
    }

    fn extract_prefix(&self, buffer: &TextBuffer, current_offset: usize) -> String {
        let mut prefix = String::new();
        let mut offset = current_offset;
        
        // Move backwards to find the start of the word
        while offset > 0 {
            let chunk = buffer.read_backward(offset);
            if chunk.is_empty() {
                break;
            }
            
            let chunk_start = offset - chunk.len();
            let mut found_word_char = false;
            
            for &byte in chunk.iter().rev() {
                let ch = byte as char;
                if ch.is_alphanumeric() || ch == '_' {
                    prefix.insert(0, ch);
                    found_word_char = true;
                } else {
                    break;
                }
            }
            
            if !found_word_char {
                break;
            }
            
            offset = chunk_start;
        }
        
        prefix
    }

    fn offset_to_point(&self, buffer: &TextBuffer, offset: usize) -> Point {
        let cursor = buffer.cursor_move_to_offset_internal(Default::default(), offset);
        cursor.logical_pos
    }

    pub fn cancel_completion(&mut self) {
        self.state.reset();
    }

    pub fn is_completing(&self) -> bool {
        self.state.is_active
    }
}

impl Default for AutoCompleter {
    fn default() -> Self {
        Self::new(Box::new(WordCompletionProvider))
    }
}