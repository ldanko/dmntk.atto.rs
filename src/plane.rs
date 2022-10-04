/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Implementation of an editing plane.

use std::fmt;
use std::fmt::Display;

const CH_WS: char = ' ';

/// Checks if the specified character is a box-drawing character.
macro_rules! is_box_drawing_character {
  ($ch:expr) => {
    match $ch {
      '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a vertical line seen from the left side.
macro_rules! is_vertical_line_left {
  ($ch:expr) => {
    match $ch {
      '│' | '├' | '║' | '╟' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a vertical line seen from the right side.
macro_rules! is_vertical_line_right {
  ($ch:expr) => {
    match $ch {
      '│' | '┤' | '║' | '╢' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a crossing with vertical line.
macro_rules! is_vertical_line_crossing {
  ($ch:expr) => {
    match $ch {
      '│' | '┼' | '┬' | '┴' | '╪' | '┐' | '┘' | '├' | '║' | '╟' | '╬' | '╥' | '╨' | '╫' | '╢' | '┤' | '╡' => true,
      _ => false,
    }
  };
}

/// Row of characters in columns.
pub struct Row {
  /// Characters in a row.
  pub columns: Vec<char>,
}

impl Display for Row {
  /// Converts a [Row] into its string representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.columns.iter().collect::<String>())
  }
}

impl Row {
  /// Creates a new row with specified characters.
  pub fn new(columns: Vec<char>) -> Self {
    Self { columns }
  }
}

/// Plane containing rows of characters.
pub struct Plane {
  /// Rows in plane.
  pub rows: Vec<Row>,
  /// Current vertical cursor position (row index).
  pub pos_row: usize,
  /// Current horizontal cursor position (column index).
  pub pos_col: usize,
  /// Vertical offset of the plane against top-left corner of the screen.
  pub offset_vert: isize,
  /// Horizontal offset of the plane against top-left corner of the screen.
  pub offset_horz: isize,
}

impl Display for Plane {
  /// Converts a [Plane] into its string representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.rows.iter().fold("".to_string(), |acc, row| format!("{}\n{}", acc, row)).trim())
  }
}

impl Plane {
  /// Creates a plane from text.
  pub fn new(content: &str) -> Self {
    let mut rows = vec![];
    for content_line in content.lines() {
      let line = content_line.trim();
      if !line.is_empty() {
        let mut columns = vec![];
        for ch in line.chars() {
          columns.push(ch);
        }
        rows.push(Row::new(columns));
      }
    }
    Self {
      rows,
      pos_row: 1,
      pos_col: 1,
      offset_vert: 0,
      offset_horz: 0,
    }
  }

  /// Returns a character *under* current cursor position.
  pub fn cur_char(&self) -> char {
    self.rows[self.pos_row].columns[self.pos_col]
  }

  /// Sets a new position of the cursor.
  pub fn move_cursor(&mut self, row_offset: i32, col_offset: i32) {
    if self.is_allowed_position(row_offset, col_offset) {
      let (row, col) = self.adjusted_position(row_offset, col_offset);
      if (1..self.rows.len() - 1).contains(&row) && (1..self.rows[row].columns.len() - 1).contains(&col) {
        self.pos_row = row;
        self.pos_col = col;
      }
    }
  }

  /// Returns the vertical position of the cursor in screen coordinates.
  pub fn cur_screen_row(&self) -> i32 {
    self.pos_row as i32 + self.offset_vert as i32
  }

  /// Returns the horizontal position of the cursor in screen coordinates.
  pub fn cur_screen_col(&self) -> i32 {
    self.pos_col as i32 + self.offset_horz as i32
  }

  /// Returns `true` when the current cursor coordinates are valid (somewhere inside decision table cell).
  pub fn is_valid_cursor_pos(&self) -> bool {
    (1..self.rows.len() - 1).contains(&self.pos_row) && (1..self.rows[self.pos_row].columns.len() - 1).contains(&self.pos_col)
  }

  /// Moves cursor up.
  pub fn move_up(&mut self) -> bool {
    if self.is_allowed_position(-1, 0) {
      self.move_cursor(-1, 0);
      return true;
    }
    if self.is_horz_line(-1, 0) && self.is_allowed_position(-2, 0) {
      self.move_cursor(-2, 0);
      return true;
    }
    false
  }

  /// Moves cursor down.
  pub fn move_down(&mut self) -> bool {
    if self.is_allowed_position(1, 0) {
      self.pos_row += 1;
      return true;
    }
    if self.is_horz_line(1, 0) && self.is_allowed_position(2, 0) {
      self.pos_row += 2;
      return true;
    }
    false
  }

  /// Moves cursor left.
  pub fn move_left(&mut self) -> bool {
    if self.is_allowed_position(0, -1) {
      self.pos_col -= 1;
      return true;
    }
    if self.is_vert_line(0, -1) && self.is_allowed_position(0, -2) {
      self.pos_col -= 2;
      return true;
    }
    false
  }

  /// Moves cursor right.
  pub fn move_right(&mut self) -> bool {
    if self.is_allowed_position(0, 1) {
      self.pos_col += 1;
      return true;
    }
    if self.is_vert_line(0, 1) && self.is_allowed_position(0, 2) {
      self.pos_col += 2;
      return true;
    }
    false
  }

  /// Places cursor at the first character in the cell (same row).
  pub fn move_cell_start(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_left() {
        self.move_cursor(0, offset + 1);
        return true;
      }
    }
    false
  }

  /// Places cursor at the last character in the cell (same row).
  pub fn move_cell_end(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_right() {
        self.move_cursor(0, offset - 1);
        return true;
      }
    }
    false
  }

  /// Places cursor at the first character in the decision table (same row).
  pub fn move_table_start(&mut self) -> bool {
    if (1..self.rows.len() - 1).contains(&self.pos_row) {
      return if is_box_drawing_character!(self.rows[self.pos_row].columns[1]) {
        self.move_cell_start()
      } else {
        self.pos_col = 1;
        true
      };
    }
    false
  }

  /// Places cursor at the last character in the cell (same row).
  pub fn move_table_end(&mut self) -> bool {
    if (1..self.rows.len() - 1).contains(&self.pos_row) {
      let index = self.rows[self.pos_row].columns.len() - 2;
      return if is_box_drawing_character!(self.rows[self.pos_row].columns[index]) {
        self.move_cell_end()
      } else {
        self.pos_col = self.rows[self.pos_row].columns.len() - 2;
        true
      };
    }
    false
  }

  /// Places cursor at the first character in the next cell (same row).
  pub fn move_cell_next(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_right() {
        return if self.is_allowed_position(0, offset + 1) {
          self.move_cursor(0, offset + 1);
          true
        } else {
          self.move_cell_end()
        };
      }
    }
    false
  }

  /// Places cursor at the last character in the previous cell (same row).
  pub fn move_cell_prev(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_left() {
        return if self.is_allowed_position(0, offset - 1) {
          self.move_cursor(0, offset - 1);
          true
        } else {
          self.move_cell_start()
        };
      }
    }
    false
  }

  /// Inserts a character at the current position.
  pub fn insert_character(&mut self, ch: char) {
    if self.is_valid_cursor_pos() {
      let (count, offset) = self.whitespaces_before_vert_line();
      let columns = &mut self.rows[self.pos_row].columns;
      columns.insert(self.pos_col, ch);
      if count > 1 {
        columns.remove(self.pos_col + offset);
      } else {
        self.a();
        self.insert_column_before_vert_line_crossing();
        self.b();
      }
      self.move_cursor(0, 1);
    }
  }

  /// Deletes a character placed *before* the cursor.
  pub fn delete_character_before_cursor(&mut self) {
    if self.is_allowed_position(0, -1) {
      self.rows[self.pos_row].columns.remove(self.pos_col - 1);
      self.move_cursor(0, -1);
      let pos = self.last_position_before_vert_line();
      if self.only_spaces_before_vert_line(pos) {
        self.a();
        self.delete_character_before_vert_line(pos);
        self.b();
      } else {
        self.insert_whitespace_before_vert_line();
      }
    }
  }

  /// Deletes a character placed *under* the cursor.
  pub fn delete_character_under_cursor(&mut self) {
    let pos = self.last_position_before_vert_line();
    if self.only_spaces_before_vert_line(pos) {
      self.a();
      self.delete_character_before_vert_line(pos);
      self.b();
    } else {
      self.insert_whitespace_before_vert_line();
    }
    self.rows[self.pos_row].columns.remove(self.pos_col);
    if is_box_drawing_character!(self.cur_char()) {
      self.move_cursor(0, -1);
    }
  }

  /// Update connection with information item name cell.
  fn a(&mut self) {
    let i = self.information_item_name_height();
    if i > 0 {
      let pos = self.rows[0].columns.len() - 1;
      if pos < self.rows[i].columns.len() {
        match self.rows[i].columns[pos] {
          '┴' => self.rows[i].columns[pos] = '─',
          '┼' => self.rows[i].columns[pos] = '┬',
          '┤' => self.rows[i].columns[pos] = '┐',
          _ => {}
        }
      }
    }
  }

  /// Update connection with information item name cell.
  fn b(&mut self) {
    let i = self.information_item_name_height();
    if i > 0 {
      let pos = self.rows[0].columns.len() - 1;
      if pos < self.rows[i].columns.len() {
        match self.rows[i].columns[pos] {
          '─' => self.rows[i].columns[pos] = '┴',
          '┬' => self.rows[i].columns[pos] = '┼',
          '┐' => self.rows[i].columns[pos] = '┤',
          _ => {}
        }
      }
    }
  }

  /// Returns the position of the last character before the vertical line.
  fn last_position_before_vert_line(&self) -> usize {
    for (pos, ch) in self.rows[self.pos_row].columns.iter().enumerate().skip(self.pos_col) {
      if is_vertical_line_left!(ch) {
        return pos - 1;
      }
    }
    self.pos_col
  }

  /// Inserts whitespace character before the next vertical line to the right from the cursor.
  fn insert_whitespace_before_vert_line(&mut self) {
    if (1..self.rows.len()).contains(&self.pos_row) {
      let row = &mut self.rows[self.pos_row];
      if (1..row.columns.len() - 1).contains(&self.pos_col) {
        for (col_index, ch) in row.columns[self.pos_col..].iter().enumerate() {
          if is_vertical_line_left!(ch) {
            row.columns.insert(self.pos_col + col_index, CH_WS);
            break;
          }
        }
      }
    }
  }

  /// Counts the number of whitespaces before the nearest vertical line to the right
  /// from current cursor position. Returns the number of whitespaces and the offset
  /// to vertical line.
  fn whitespaces_before_vert_line(&self) -> (usize, usize) {
    let mut count = 0;
    let mut offset = 0;
    for ch in &self.rows[self.pos_row].columns[self.pos_col + 1..] {
      if is_vertical_line_left!(ch) {
        break;
      } else if *ch == CH_WS {
        count += 1;
      } else {
        count = 0;
      }
      offset += 1;
    }
    (count, offset)
  }

  ///
  fn insert_column_before_vert_line_crossing(&mut self) {
    let (skip, take) = self.rows_skip_take();
    for (row_index, row) in self.rows.iter_mut().enumerate().skip(skip).take(take) {
      if row_index != self.pos_row && self.pos_col < row.columns.len() - 1 {
        let mut found_char = CH_WS;
        let mut found_index = 0;
        for (col_index, ch) in row.columns[self.pos_col..].iter().enumerate() {
          if is_vertical_line_crossing!(ch) {
            found_char = *ch;
            found_index = self.pos_col + col_index;
            break;
          }
        }
        match found_char {
          '│' | '├' | '║' | '╟' => row.columns.insert(found_index, CH_WS),
          '┼' | '┬' | '┴' | '┐' | '┘' | '┤' | '╥' | '╨' | '╫' | '╢' => row.columns.insert(found_index, '─'),
          '╪' | '╬' | '╡' => row.columns.insert(found_index, '═'),
          _ => {}
        }
      }
    }
  }

  /// Returns `true` if there is a whitespace is before the next
  /// vertical line to the right from the specified position.
  fn only_spaces_before_vert_line(&self, pos: usize) -> bool {
    let (skip, take) = self.rows_skip_take();
    for (row_index, row) in self.rows.iter().enumerate().skip(skip).take(take) {
      // check if the current column is not after the end of each row
      if (1..row.columns.len() - 1).contains(&pos) {
        // check the character at column position, if box-drawing then skip
        let ch = self.rows[row_index].columns[pos];
        if !is_box_drawing_character!(ch) {
          // move to the right until vertical line is found
          for chars in row.columns[pos - 1..].windows(3) {
            if is_vertical_line_left!(chars[2]) {
              // if there is no whitespace before vertical line,
              // no further checking is needed, just return `false`
              if chars[1] != CH_WS {
                return false;
              }
              // if there is a whitespace, but just between two box-drawing
              // characters, no further checking is needed, just return `false`
              if is_box_drawing_character!(chars[0]) {
                return false;
              }
              // whitespace found, check the next row
              break;
            }
          }
        }
      }
    }
    true
  }

  /// Deletes a single character before the next vertical line to the right
  /// from the specified position.
  fn delete_character_before_vert_line(&mut self, pos: usize) {
    let (skip, take) = self.rows_skip_take();
    for (row_index, row) in self.rows.iter_mut().enumerate().skip(skip).take(take) {
      if row_index != self.pos_row && pos < row.columns.len() - 1 {
        let mut found_index = 0;
        for (col_index, ch) in row.columns[pos..].iter().enumerate() {
          if is_vertical_line_crossing!(ch) {
            found_index = pos + col_index;
            break;
          }
        }
        if found_index > 0 {
          row.columns.remove(found_index - 1);
        }
      }
    }
  }

  /// Returns `true` when the character at the specified position is a horizontal line.
  fn is_horz_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '─' | '═')
    } else {
      false
    }
  }

  /// Returns `true` when the character at the specified position is a vertical line.
  fn is_vert_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '│' | '║')
    } else {
      false
    }
  }

  /// Returns `true` when the cursor position is allowed according to horizontal and vertical offset.
  fn is_allowed_position(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r > 0 && r < self.rows.len() - 1 && c > 0 && c < self.rows[r].columns.len() - 1 {
      !is_box_drawing_character!(self.rows[r].columns[c])
    } else {
      false
    }
  }

  /// Calculates a new position according the specified row and column offset.
  fn adjusted_position(&self, row_offset: i32, col_offset: i32) -> (usize, usize) {
    (
      if row_offset >= 0 {
        self.pos_row.saturating_add(row_offset as usize)
      } else {
        self.pos_row.saturating_sub(row_offset.unsigned_abs() as usize)
      },
      if col_offset >= 0 {
        self.pos_col.saturating_add(col_offset as usize)
      } else {
        self.pos_col.saturating_sub(col_offset.unsigned_abs() as usize)
      },
    )
  }

  /// Returns the offset of the vertical line to the right from current cursor position.
  fn get_vert_line_offset_right(&self) -> Option<i32> {
    if self.is_valid_cursor_pos() {
      for (index, ch) in self.rows[self.pos_row].columns[self.pos_col..].iter().enumerate() {
        if is_vertical_line_left!(ch) {
          if let Ok(offset) = index.try_into() {
            return Some(offset);
          }
        }
      }
    }
    None
  }

  /// Returns the offset of the vertical line to the left from current cursor position.
  fn get_vert_line_offset_left(&self) -> Option<i32> {
    if self.is_valid_cursor_pos() {
      for (offset, ch) in self.rows[self.pos_row].columns[0..=self.pos_col].iter().rev().enumerate() {
        if is_vertical_line_right!(ch) {
          return Some(-(offset as i32));
        }
      }
    }
    None
  }

  ///
  fn rows_skip_take(&self) -> (usize, usize) {
    let height = self.information_item_name_height();
    if self.pos_row < height {
      (0, height - 1)
    } else {
      (height, self.rows.len() - height)
    }
  }

  /// Returns the height of the information item name.
  fn information_item_name_height(&self) -> usize {
    for (row_index, row) in self.rows.iter().enumerate() {
      //TODO may be optimized, search in rows starting with '┌' or '├'
      for ch in &row.columns {
        if *ch == '╥' {
          return row_index;
        }
      }
    }
    0
  }
}
