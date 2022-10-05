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

//! Implementation of a minimalistic decision table editor.

extern crate ncurses;

mod errors;
mod keys;
mod plane;
#[cfg(test)]
mod tests;

use errors::*;
use keys::*;
use ncurses::*;
use plane::*;
use std::{env, fs};

/// Editor action.
enum Action {
  CursorMoveCellStart,
  CursorMoveCellEnd,
  CursorMoveCellLeft,
  CursorMoveCellRight,
  CursorMoveDown,
  CursorMoveLeft,
  CursorMoveRight,
  CursorMoveTableStart,
  CursorMoveTableEnd,
  CursorMoveUp,
  DebugKeystroke(i32, String),
  DeleteChar,
  DeleteCharBefore,
  DoNothing,
  InsertChar(char),
  ResizeWindow,
  Quit,
}

///
struct Editor {
  /// Handle of the main window of the terminal.
  window: WINDOW,
  plane: Plane,
}

impl Editor {
  /// Creates a new editor initialized with the content loaded from file.
  fn new(file_name: &str) -> Result<Self, AttoError> {
    let content = fs::read_to_string(file_name).map_err(|e| err_load_file(file_name, &e.to_string()))?;
    let plane = Plane::new(&content);
    let window = Self::initialize();
    Ok(Self { window, plane })
  }

  /// Initializes terminal via ncurses.
  fn initialize() -> WINDOW {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");
    let window = initscr();
    raw();
    keypad(window, true);
    noecho();
    window
  }

  /// Terminates terminal via ncurses.
  fn finalize(&self) -> Result<()> {
    endwin();
    Ok(())
  }

  /// Updates cursor position.
  fn update_cursor(&self) {
    mv(self.plane.cursor_row() as i32, self.plane.cursor_col() as i32);
  }

  /// Updates cursor coordinates in status bar.
  fn update_cursor_coordinates(&self) {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(self.window, &mut max_y, &mut max_x);
    getyx(self.window, &mut cur_y, &mut cur_x);
    mvaddstr(
      max_y - 1,
      max_x - 20,
      &format!("{:>20}", format!("{}:{} ", self.plane.cursor_col(), self.plane.cursor_row())),
    );
    mv(cur_y, cur_x);
  }

  /// Repaints the content of a plane.
  fn repaint_plane(&self) {
    for (r, row) in self.plane.rows().iter().enumerate() {
      mv(r as i32, 0);
      addstr(&row.to_string());
      addstr("  ");
    }
  }

  /// Maps a key-stroke to editor action.
  fn map_key_to_action(&self, key: i32) -> Action {
    if let Some(key_name) = keyname(key) {
      match key_name.as_str() {
        KN_CTRL_Q => Action::Quit,
        KN_UP => Action::CursorMoveUp,
        KN_DOWN => Action::CursorMoveDown,
        KN_LEFT => Action::CursorMoveLeft,
        KN_RIGHT => Action::CursorMoveRight,
        KN_BACKSPACE => Action::DeleteCharBefore,
        KN_DELETE => Action::DeleteChar,
        KN_HOME => Action::CursorMoveCellStart,
        KN_END => Action::CursorMoveCellEnd,
        KN_SHIFT_HOME => Action::CursorMoveTableStart,
        KN_SHIFT_END => Action::CursorMoveTableEnd,
        KN_TAB => Action::CursorMoveCellRight,
        KN_SHIFT_TAB => Action::CursorMoveCellLeft,
        KN_RESIZE => Action::ResizeWindow,
        _ => match key {
          32..=126 => Action::InsertChar(char::from_u32(key as u32).unwrap()),
          127 => Action::DeleteChar,
          _ => Action::DebugKeystroke(key, key_name),
        },
      }
    } else {
      Action::DoNothing
    }
  }

  /// Processes input key-strokes.
  fn process_keystrokes(&mut self) {
    loop {
      match self.map_key_to_action(getch()) {
        Action::CursorMoveCellStart => {
          if self.plane.cursor_move_cell_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveCellEnd => {
          if self.plane.cursor_move_cell_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveCellLeft => {
          if self.plane.cursor_move_cell_left() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveCellRight => {
          if self.plane.cursor_move_cell_right() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveDown => {
          if self.plane.cursor_move_down() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveLeft => {
          if self.plane.cursor_move_left() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveRight => {
          if self.plane.cursor_move_right() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveTableStart => {
          if self.plane.cursor_move_table_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveTableEnd => {
          if self.plane.cursor_move_table_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::CursorMoveUp => {
          if self.plane.cursor_move_up() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        Action::DebugKeystroke(key, key_name) => {
          let mut x = 0;
          let mut y = 0;
          let mut mx = 0;
          let mut my = 0;
          getyx(self.window, &mut y, &mut x);
          getmaxyx(self.window, &mut my, &mut mx);
          mvaddstr(my - 1, 30, &format!("{} | {:40}", key, key_name));
          mv(y, x);
          refresh();
        }
        Action::DeleteChar => {
          self.plane.delete_char();
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
        }
        Action::DeleteCharBefore => {
          self.plane.delete_char_before();
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        Action::DoNothing => {}
        Action::InsertChar(ch) => {
          self.plane.insert_char(ch);
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        Action::ResizeWindow => {
          // getmaxyx(self.window, &mut max_y, &mut max_x);
          // getyx(window, &mut cur_y, &mut cur_x);
          // attron(A_REVERSE());
          // mvaddstr(43, 1, &format!("{}:{}", max_x, max_y));
          // attroff(A_REVERSE());
          // mv(cur_y, cur_x);
          // refresh();
        }
        Action::Quit => break,
      }
    }
  }
}

/// Prints usage message.
fn usage() {
  println!("usage")
}

/// Temporary debug function.
//TODO remove
pub fn debug(msg: &str) {
  let mut x = 0;
  let mut y = 0;
  let mut mx = 0;
  let mut my = 0;
  getyx(stdscr(), &mut y, &mut x);
  getmaxyx(stdscr(), &mut my, &mut mx);
  mvaddstr(my - 1, 1, msg);
  mv(y, x);
  refresh();
}

/// Main entrypoint.
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    usage();
    return Err(err_invalid_arguments());
  }
  let mut editor = Editor::new(&args[1])?;
  editor.repaint_plane();
  editor.update_cursor();
  editor.update_cursor_coordinates();
  refresh();
  editor.process_keystrokes();
  editor.finalize()
}
