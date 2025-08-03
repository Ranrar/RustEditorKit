//! Search and replace functionality for EditorBuffer
//!
//! This module contains text search, find/replace, and match highlighting operations.

use super::buffer::EditorBuffer;

/// Search match result
#[derive(Debug, Clone, PartialEq)]
pub struct SearchMatch {
    pub row: usize,
    pub col: usize,
    pub length: usize,
}

impl EditorBuffer {
    /// Find the next occurrence of a string, returns (row, col) or None
    pub fn find_next(&self, query: &str, from: Option<(usize, usize)>) -> Option<(usize, usize)> {
        if query.is_empty() {
            return None;
        }

        let (row, col) = from.unwrap_or((self.cursor.row, self.cursor.col));
        
        // Search from current position to end of buffer
        for r in row..self.lines.len() {
            let start = if r == row { col } else { 0 };
            if let Some(idx) = self.lines[r][start..].find(query) {
                return Some((r, start + idx));
            }
        }
        
        // If not found, wrap around and search from beginning
        for r in 0..=row {
            let end = if r == row { col } else { self.lines[r].len() };
            if let Some(idx) = self.lines[r][..end].find(query) {
                return Some((r, idx));
            }
        }
        
        None
    }

    /// Find the previous occurrence of a string
    pub fn find_previous(&self, query: &str, from: Option<(usize, usize)>) -> Option<(usize, usize)> {
        if query.is_empty() {
            return None;
        }

        let (mut row, mut col) = from.unwrap_or((self.cursor.row, self.cursor.col));
        
        // Search from current position backward
        loop {
            let search_text = if row < self.lines.len() {
                &self.lines[row][..col.min(self.lines[row].len())]
            } else {
                break;
            };
            
            if let Some(idx) = search_text.rfind(query) {
                return Some((row, idx));
            }
            
            if row == 0 {
                break;
            }
            
            row -= 1;
            col = self.lines[row].len();
        }
        
        None
    }

    /// Find all occurrences of a string in the buffer
    pub fn find_all(&self, query: &str) -> Vec<SearchMatch> {
        if query.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();
        
        for (row, line) in self.lines.iter().enumerate() {
            let mut start = 0;
            while let Some(idx) = line[start..].find(query) {
                let col = start + idx;
                matches.push(SearchMatch {
                    row,
                    col,
                    length: query.len(),
                });
                start = col + 1; // Move past this match to find overlapping matches
            }
        }
        
        matches
    }

    /// Replace the next occurrence of a string
    pub fn replace_next(&mut self, query: &str, replacement: &str, from: Option<(usize, usize)>) -> bool {
        if let Some((row, col)) = self.find_next(query, from) {
            self.push_undo();
            let line = &mut self.lines[row];
            line.replace_range(col..col + query.len(), replacement);
            self.cursor.row = row;
            self.cursor.col = col + replacement.len();
            println!("[DEBUG] Replaced '{}' with '{}' at ({}, {})", query, replacement, row, col);
            return true;
        }
        false
    }

    /// Replace all occurrences of a string
    pub fn replace_all(&mut self, query: &str, replacement: &str) -> usize {
        if query.is_empty() {
            return 0;
        }

        self.push_undo();
        let mut count = 0;
        
        for line in &mut self.lines {
            let new_line = line.replace(query, replacement);
            if new_line != *line {
                let replacements = line.matches(query).count();
                count += replacements;
                *line = new_line;
            }
        }
        
        println!("[DEBUG] Replaced {} occurrences of '{}' with '{}'", count, query, replacement);
        count
    }

    /// Jump to a specific search match
    pub fn goto_search_match(&mut self, search_match: &SearchMatch) {
        self.cursor.row = search_match.row;
        self.cursor.col = search_match.col;
        
        // Optionally select the match
        let mut sel = crate::corelogic::selection::Selection::new(search_match.row, search_match.col);
        sel.set(
            search_match.row, 
            search_match.col, 
            search_match.row, 
            search_match.col + search_match.length
        );
        self.selection = Some(sel);
        
        println!("[DEBUG] Jumped to search match at ({}, {})", search_match.row, search_match.col);
    }

    /// Search with case sensitivity option
    pub fn find_next_case_sensitive(&self, query: &str, case_sensitive: bool, from: Option<(usize, usize)>) -> Option<(usize, usize)> {
        if case_sensitive {
            self.find_next(query, from)
        } else {
            self.find_next(&query.to_lowercase(), from)
        }
    }

    /// Check if the text at a position matches the query
    pub fn text_matches_at(&self, row: usize, col: usize, query: &str) -> bool {
        if row >= self.lines.len() || col >= self.lines[row].len() {
            return false;
        }
        
        let line = &self.lines[row];
        if col + query.len() > line.len() {
            return false;
        }
        
        &line[col..col + query.len()] == query
    }

    /// Get context around a search match (for search results display)
    pub fn get_search_context(&self, search_match: &SearchMatch, context_chars: usize) -> String {
        let line = &self.lines[search_match.row];
        let start = search_match.col.saturating_sub(context_chars);
        let end = (search_match.col + search_match.length + context_chars).min(line.len());
        
        format!("{}**{}**{}", 
            &line[start..search_match.col],
            &line[search_match.col..search_match.col + search_match.length],
            &line[search_match.col + search_match.length..end]
        )
    }
}
