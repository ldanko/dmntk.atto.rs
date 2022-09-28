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

/// Row of characters.
#[derive(Debug, Default, Clone)]
pub struct Row {
  /// Characters in a row.
  pub columns: Vec<char>,
}

impl ToString for Row {
  /// Converts [Row] into its string representation.
  fn to_string(&self) -> String {
    self.columns.iter().collect()
  }
}

/// Plane containing rows of characters.
#[derive(Debug, Default, Clone)]
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

impl Plane {
  /// Creates a plane from text.
  pub fn new(content: &str) -> Self {
    let mut rows = vec![];
    for content_line in content.lines() {
      let line = content_line.trim();
      if !line.is_empty() {
        let mut row = Row::default();
        for ch in line.chars() {
          row.columns.push(ch);
        }
        rows.push(row);
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
      self.pos_y -= 1;
      return true;
    }
    if self.is_horz_line(-1, 0) && self.is_allowed(-2, 0) {
      self.pos_y -= 2;
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
      // check if the character deletion in other rows is possible
      let mut can_delete = true;
      for (row_index, row) in self.rows.iter_mut().enumerate() {
        if row_index != self.pos_y && self.pos_x < row.columns.len() - 1 {
          for (col_index, ch) in row.columns[self.pos_x..].iter().enumerate() {
            if matches!(ch, '│' | '├' | '║' | '╟') {
              if row.columns[self.pos_x + col_index - 1] != ' ' {
                can_delete = false;
              }
              break;
            }
          }
        }
      }
      if can_delete {
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
      } else {
        self.rows[self.pos_y].columns.insert(self.pos_x + 1, ' ');
      }
    }
  }
  /// Returns `true` when the character at the specified position is a horizontal line.
  fn is_horz_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.apply_offset(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '─' | '═')
    } else {
      false
    }
  }
  /// Returns `true` when the character at the specified position is a vertical line.
  fn is_vert_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.apply_offset(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '│' | '║')
    } else {
      false
    }
  }
  /// Returns `true` when the cursor position is allowed according to horizontal and vertical offset.
  fn is_allowed(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.apply_offset(row_offset, col_offset);
    if r > 0 && r < self.rows.len() - 1 && c > 0 && c < self.rows[r].columns.len() - 1 {
      self.is_box_drawing_char(self.rows[r].columns[c])
    } else {
      false
    }
  }
  /// Calculates new position according the specified offset.
  fn apply_offset(&self, row_offset: i32, col_offset: i32) -> (usize, usize) {
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
  /// Returns `true` when the character is not a box-drawing character.
  #[rustfmt::skip]
  fn is_box_drawing_char(&self, ch: char) -> bool {
    !matches!(ch, '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢')
  }
}
