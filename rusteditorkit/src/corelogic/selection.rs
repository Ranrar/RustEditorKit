/// Selection logic for EditorBuffer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
	pub start_row: usize,
	pub start_col: usize,
	pub end_row: usize,
	pub end_col: usize,
}

impl Selection {
	/// Set selection start and end
	pub fn set(&mut self, start_row: usize, start_col: usize, end_row: usize, end_col: usize) {
		self.start_row = start_row;
		self.start_col = start_col;
		self.end_row = end_row;
		self.end_col = end_col;
	}
	pub fn new(start_row: usize, start_col: usize) -> Self {
		Self {
			start_row,
			start_col,
			end_row: start_row,
			end_col: start_col,
		}
	}

	/// Returns (start_row, start_col, end_row, end_col) in normalized order
	pub fn normalized(&self) -> ((usize, usize), (usize, usize)) {
		if (self.start_row, self.start_col) <= (self.end_row, self.end_col) {
			((self.start_row, self.start_col), (self.end_row, self.end_col))
		} else {
			((self.end_row, self.end_col), (self.start_row, self.start_col))
		}
	}

	/// Returns the selection range as (start_row, start_col, end_row, end_col)
	pub fn range(&self) -> (usize, usize, usize, usize) {
		let ((start_row, start_col), (end_row, end_col)) = self.normalized();
		(start_row, start_col, end_row, end_col)
	}

	/// Returns true if selection is active (not collapsed)
	pub fn is_active(&self) -> bool {
		(self.start_row, self.start_col) != (self.end_row, self.end_col)
	}

	/// Clamp selection to buffer lines
	pub fn clamp_to_buffer(&mut self, lines: &[String]) {
		let max_row = lines.len().saturating_sub(1);
		self.start_row = self.start_row.min(max_row);
		self.end_row = self.end_row.min(max_row);
		self.start_col = self.start_col.min(lines[self.start_row].chars().count());
		self.end_col = self.end_col.min(lines[self.end_row].chars().count());
	}

	/// Get selected text for copy
	pub fn get_selected_text(&self, lines: &[String]) -> String {
		let (start_row, start_col, end_row, end_col) = self.range();
		if start_row == end_row {
			let line = &lines[start_row];
			line.chars().skip(start_col).take(end_col - start_col).collect()
		} else {
			let mut result = String::new();
			// First line
			result.push_str(&lines[start_row].chars().skip(start_col).collect::<String>());
			result.push('\n');
			// Middle lines
			for row in (start_row + 1)..end_row {
				result.push_str(&lines[row]);
				result.push('\n');
			}
			// Last line
			result.push_str(&lines[end_row].chars().take(end_col).collect::<String>());
			result
		}
	}
}
