mod physics;
mod collision;
mod hole;
mod game;

use game::GameState;
use hole::course;
use std::sync::Mutex;

type Game = Mutex<GameState>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(Mutex::new(GameState::new(course())))
    .invoke_handler(tauri::generate_handler![
      init_game,
      update_game,
      strike_ball,
      reset_game,
      next_hole,
      get_game_state,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn init_game(state: tauri::State<Game>) -> GameState {
  state.lock().unwrap().clone()
}

#[tauri::command]
fn update_game(state: tauri::State<Game>, dt: f32) -> GameState {
  let mut game = state.lock().unwrap();
  game.update(dt);
  game.clone()
}

#[tauri::command]
fn strike_ball(state: tauri::State<Game>, angle: f32, power: f32) -> bool {
  state.lock().unwrap().strike(angle, power)
}

#[tauri::command]
fn reset_game(state: tauri::State<Game>) -> GameState {
  let mut game = state.lock().unwrap();
  game.reset();
  game.clone()
}

#[tauri::command]
fn next_hole(state: tauri::State<Game>) -> GameState {
  let mut game = state.lock().unwrap();
  game.next_hole();
  game.clone()
}

#[tauri::command]
fn get_game_state(state: tauri::State<Game>) -> GameState {
  state.lock().unwrap().clone()
}
