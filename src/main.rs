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

mod dev_utils;
mod editor;
mod errors;
mod keys;
mod plane;
#[cfg(test)]
mod tests;

use editor::Editor;
use errors::*;
use ncurses::*;
use std::env;

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
  editor.repaint_plane();
  editor.update_cursor();
  editor.update_cursor_coordinates();
  refresh();
  editor.process_keystrokes();
  editor.finalize()
}
