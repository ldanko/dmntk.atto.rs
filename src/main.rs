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

mod keys;
mod plane;
#[cfg(test)]
mod tests;

use keys::*;
use ncurses::*;
use plane::*;
use std::{env, fs};

/// Initializes ncurses.
fn initialize() {
  // set locale to Unicode en-US
  let locale_conf = LcCategory::all;
  setlocale(locale_conf, "en_US.UTF-8");
  // start ncurses
  initscr();
  // switch to raw mode
  raw();
  // allow for extended keyboard (like F1)
  keypad(stdscr(), true);
  // disable echo
  noecho();
}

/// Terminates ncurses.
fn finalize() {
  // terminate ncurses
  endwin();
}

/// Loads the input file.
fn load_from_file(file_name: &str) -> Option<Plane> {
  if let Ok(content) = fs::read_to_string(file_name) {
    Some(Plane::new(&content))
  } else {
    eprintln!("Loading file failed: {}", file_name);
    None
  }
}

fn repaint_plane(plane: &Plane) {
  for (r, row) in plane.rows.iter().enumerate() {
    mv(r as i32, 0);
    addstr(&row.to_string());
    addstr(" ");
  }
  mv(plane.cur_screen_row(), plane.cur_screen_col());
  refresh();
}

/// Processes input keystrokes.
fn process_keystrokes(plane: &mut Plane) {
  let mut cur_x = 0;
  let mut cur_y = 0;
  for row in &plane.rows {
    mv(cur_y, cur_x);
    addstr(&row.to_string());
    cur_y += 1;
  }
  cur_x = plane.cur_screen_col();
  cur_y = plane.cur_screen_row();
  mv(cur_y, cur_x);
  loop {
    let ch = getch();
    let key_name = keyname(ch).unwrap_or_default();
    match key_name.as_str() {
      KN_CTRL_Q => break,
      KN_UP => {
        if plane.move_up() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_DOWN => {
        if plane.move_down() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_LEFT => {
        if plane.move_left() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_RIGHT => {
        if plane.move_right() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_BACKSPACE => {
        plane.delete_character_before();
        repaint_plane(plane);
      }
      KN_DEL => {
        plane.delete_character();
        repaint_plane(plane);
      }
      KN_HOME => {
        if plane.move_cell_start() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_END => {
        if plane.move_cell_end() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_SHIFT_HOME => {
        if plane.move_table_start() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_SHIFT_END => {
        if plane.move_table_end() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_TAB => {
        if plane.move_cell_next() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      KN_SHIFT_TAB => {
        if plane.move_cell_prev() {
          mv(plane.cur_screen_row(), plane.cur_screen_col());
        }
      }
      _ => match ch {
        32..=127 => {
          if let Some(new_ch) = char::from_u32(ch as u32) {
            plane.insert_character(new_ch);
            repaint_plane(plane);
          }
        }
        _ => {
          getyx(stdscr(), &mut cur_y, &mut cur_x);
          mvaddstr(40, 1, &format!("{:X}", ch));
          mvaddstr(41, 1, &format!("{:40}", key_name));
          mv(cur_y, cur_x);
          refresh();
        }
      },
    }
  }
}

/// Prints usage message.
fn usage() {
  println!("usage")
}

/// Main entrypoint.
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    usage();
    return;
  }
  initialize();
  if let Some(mut plane) = load_from_file(&args[1]) {
    process_keystrokes(&mut plane);
  }
  finalize();
}
