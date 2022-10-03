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
    mv(self.plane.cur_screen_row(), self.plane.cur_screen_col());
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
      &format!("{:>20}", format!("{}:{} ", self.plane.cur_screen_col(), self.plane.cur_screen_row())),
    );
    mv(cur_y, cur_x);
  }
  /// Repaints the content of a plane.
  fn repaint_plane(&self) {
    for (r, row) in self.plane.rows.iter().enumerate() {
      mv(r as i32, 0);
      addstr(&row.to_string());
      addstr(" ");
    }
  }
  /// Processes input keystrokes.
  fn process_keystrokes(&mut self) {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for row in &self.plane.rows {
      mv(cur_y, cur_x);
      addstr(&row.to_string());
      cur_y += 1;
    }
    self.update_cursor();
    self.update_cursor_coordinates();
    refresh();
    loop {
      let ch = getch();
      let key_name = keyname(ch).unwrap_or_default();
      match key_name.as_str() {
        KN_CTRL_Q => break,
        KN_UP => {
          if self.plane.move_up() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_DOWN => {
          if self.plane.move_down() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_LEFT => {
          if self.plane.move_left() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_RIGHT => {
          if self.plane.move_right() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_BACKSPACE => {
          self.plane.delete_character_before(true);
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        KN_DELETE => {
          self.plane.delete_character(true);
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
        }
        KN_SHIFT_DELETE => {
          self.plane.delete_character(false);
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        KN_HOME => {
          if self.plane.move_cell_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_END => {
          if self.plane.move_cell_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_SHIFT_HOME => {
          if self.plane.move_table_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_SHIFT_END => {
          if self.plane.move_table_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_TAB => {
          if self.plane.move_cell_next() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_SHIFT_TAB => {
          if self.plane.move_cell_prev() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        KN_RESIZE => {
          getmaxyx(self.window, &mut max_y, &mut max_x);
          // getyx(window, &mut cur_y, &mut cur_x);
          // attron(A_REVERSE());
          // mvaddstr(43, 1, &format!("{}:{}", max_x, max_y));
          // attroff(A_REVERSE());
          // mv(cur_y, cur_x);
          // refresh();
          a(self.window, max_x, max_y, self.plane.cur_screen_col(), self.plane.cur_screen_row());
        }
        _ => match ch {
          32..=126 => {
            if let Some(new_ch) = char::from_u32(ch as u32) {
              self.plane.insert_character(new_ch);
              self.repaint_plane();
              self.update_cursor();
              self.update_cursor_coordinates();
              refresh();
            }
          }
          _ => {
            getyx(self.window, &mut cur_y, &mut cur_x);
            mvaddstr(40, 1, &format!("{:X}", ch));
            mvaddstr(41, 1, &format!("{:40}", key_name));
            mv(cur_y, cur_x);
            refresh();
          }
        },
      }
    }
  }
}

fn a(window: WINDOW, max_x: i32, max_y: i32, screen_x: i32, screen_y: i32) {
  let mut cur_x = 0;
  let mut cur_y = 0;
  getyx(window, &mut cur_y, &mut cur_x);
  let s = format!("{:>width$}", format!("{}:{} ", screen_x, screen_y), width = max_x as usize);
  attron(A_REVERSE());
  mvaddstr(max_y - 1, 0, &s);
  attroff(A_REVERSE());
  mv(cur_y, cur_x);
  refresh();
}

/// Prints usage message.
fn usage() {
  println!("usage")
}

/// Main entrypoint.
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    usage();
    return Err(err_invalid_arguments());
  }
  let mut editor = Editor::new(&args[1])?;
  editor.process_keystrokes();
  editor.finalize()
}
