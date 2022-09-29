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

///
macro_rules! is_box_drawing_char {
  ($ch:expr) => {
    match $ch {
      '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢' => true,
      _ => false,
    }
  };
}

///
macro_rules! is_vert_line_left {
  ($ch:expr) => {
    match $ch {
      '│' | '├' | '║' | '╟' => true,
      _ => false,
    }
  };
}

/// Row of characters.
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
  /// Current horizontal cursor position.
  pub pos_x: usize,
  /// Current vertical cursor position.
  pub pos_y: usize,
  /// Horizontal offset of the plane against top-left corner of the screen.
  pub offset_x: isize,
  /// Vertical offset of the plane against top-left corner of the screen.
  pub offset_y: isize,
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
      pos_x: 1,
      pos_y: 1,
      offset_x: 0,
      offset_y: 0,
    }
  }
  /// Sets a new position of the cursor.
  pub fn move_cursor(&mut self, row_offset: i32, col_offset: i32) {
    if self.is_allowed(row_offset, col_offset) {
      let (row, col) = self.adjusted_position(row_offset, col_offset);
      if (1..self.rows.len() - 1).contains(&row) && (1..self.rows[row].columns.len() - 1).contains(&col) {
        self.pos_y = row;
        self.pos_x = col;
      }
    }
  }
  /// Returns the horizontal position of the cursor in screen coordinates.
  pub fn cur_screen_x(&self) -> i32 {
    self.pos_x as i32 + self.offset_x as i32
  }
  /// Returns the vertical position of the cursor in screen coordinates.
  pub fn cur_screen_y(&self) -> i32 {
    self.pos_y as i32 + self.offset_y as i32
  }
  /// Moves cursor up.
  pub fn move_up(&mut self) -> bool {
    if self.is_allowed(-1, 0) {
      self.move_cursor(-1, 0);
      return true;
    }
    if self.is_horz_line(-1, 0) && self.is_allowed(-2, 0) {
      self.move_cursor(-2, 0);
      return true;
    }
    false
  }
  /// Moves cursor down.
  pub fn move_down(&mut self) -> bool {
    if self.is_allowed(1, 0) {
      self.pos_y += 1;
      return true;
    }
    if self.is_horz_line(1, 0) && self.is_allowed(2, 0) {
      self.pos_y += 2;
      return true;
    }
    false
  }
  /// Moves cursor left.
  pub fn move_left(&mut self) -> bool {
    if self.is_allowed(0, -1) {
      self.pos_x -= 1;
      return true;
    }
    if self.is_vert_line(0, -1) && self.is_allowed(0, -2) {
      self.pos_x -= 2;
      return true;
    }
    false
  }
  /// Moves cursor right.
  pub fn move_right(&mut self) -> bool {
    if self.is_allowed(0, 1) {
      self.pos_x += 1;
      return true;
    }
    if self.is_vert_line(0, 1) && self.is_allowed(0, 2) {
      self.pos_x += 2;
      return true;
    }
    false
  }
  /// Inserts a character after current position.
  pub fn insert_character(&mut self, ch: char) {
    self.rows[self.pos_y].columns.insert(self.pos_x, ch);
    self.pos_x += 1;
    for (row_index, row) in self.rows.iter_mut().enumerate() {
      if row_index != self.pos_y && self.pos_x < row.columns.len() - 1 {
        let mut found_char = ' ';
        let mut found_index = 0;
        for (col_index, ch) in row.columns[self.pos_x..].iter().enumerate() {
          if matches!(
            ch,
            '│' | '┼' | '┬' | '┴' | '╪' | '┐' | '┘' | '├' | '║' | '╟' | '╬' | '╥' | '╨' | '╫' | '╢' | '┤' | '╡'
          ) {
            found_char = *ch;
            found_index = self.pos_x + col_index;
            break;
          }
        }
        match found_char {
          '│' | '├' | '║' | '╟' => row.columns.insert(found_index, ' '),
          '┼' | '┬' | '┴' | '┐' | '┘' | '┤' | '╥' | '╨' | '╫' | '╢' => row.columns.insert(found_index, '─'),
          '╪' | '╬' | '╡' => row.columns.insert(found_index, '═'),
          _ => {}
        }
      }
    }
  }
  /// Deletes a character.
  pub fn delete_character(&mut self, before: bool) {
    let mut deleted = false;
    if before {
      if !self.is_vert_line(0, -1) {
        self.rows[self.pos_y].columns.remove(self.pos_x - 1);
        self.pos_x -= 1;
        deleted = true;
      }
    } else {
      self.rows[self.pos_y].columns.remove(self.pos_x);
      deleted = true;
    }
    if deleted {
      if self.can_vert_delete() {
        self.vert_delete();
      } else {
        self.rows[self.pos_y].columns.insert(self.pos_x + 1, ' ');
      }
    }
  }
  /// Returns `true` when deletion before vertical line is possible.
  ///
  /// (describe in details)
  ///
  fn can_vert_delete(&self) -> bool {
    for (row_index, row) in self.rows.iter().enumerate() {
      if row_index != self.pos_y && (1..row.columns.len() - 1).contains(&self.pos_x) {
        for (col_index, ch) in row.columns[self.pos_x..].iter().enumerate() {
          if is_vert_line_left!(ch) {
            if row.columns[self.pos_x + col_index - 1] != CH_WS {
              return false;
            }
            break;
          }
        }
      }
    }
    true
  }
  ///
  fn vert_delete(&mut self) {
    for (row_index, row) in self.rows.iter_mut().enumerate() {
      if row_index != self.pos_y && self.pos_x < row.columns.len() - 1 {
        let mut found_index = 0;
        for (col_index, ch) in row.columns[self.pos_x..].iter().enumerate() {
          if matches!(
            ch,
            '│' | '┼' | '┬' | '┴' | '╪' | '┐' | '┘' | '├' | '║' | '╟' | '╬' | '╥' | '╨' | '╫' | '╢' | '┤' | '╡'
          ) {
            found_index = self.pos_x + col_index;
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
  fn is_allowed(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r > 0 && r < self.rows.len() - 1 && c > 0 && c < self.rows[r].columns.len() - 1 {
      !is_box_drawing_char!(self.rows[r].columns[c])
    } else {
      false
    }
  }
  /// Calculates new position adjusted with the specified row and column offset.
  fn adjusted_position(&self, row_offset: i32, col_offset: i32) -> (usize, usize) {
    (
      if row_offset >= 0 {
        self.pos_y.saturating_add(row_offset.unsigned_abs() as usize)
      } else {
        self.pos_y.saturating_sub(row_offset.unsigned_abs() as usize)
      },
      if col_offset >= 0 {
        self.pos_x.saturating_add(col_offset.unsigned_abs() as usize)
      } else {
        self.pos_x.saturating_sub(col_offset.unsigned_abs() as usize)
      },
    )
  }
}
