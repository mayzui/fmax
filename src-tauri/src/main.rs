#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::Window;
use std::{thread,time};
use tauri::Manager;

#[tauri::command]
fn custom_command(a : f32, b: f32)-> f32 {
  println!("开始计算:");
  let c = a * b;
  c
}

#[tauri::command]
fn init_process(window: Window) {
  std::thread::spawn(move || {
    loop {
       window.emit(
            "event-name",
            Payload {
             message: "全局事件!".into()
            }
       ).unwrap();
      thread::sleep(time::Duration::from_millis(5000));
    }
  });
}

#[tauri::command]

async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("beforeMain") {
    splashscreen.close().unwrap();
    window.get_window("main").unwrap().show().unwrap();
    window.get_window("main").unwrap().set_focus().unwrap();
  }
    // Show main window
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}







use tauri::{SystemTray, SystemTrayMenu,SystemTrayEvent};

fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
          SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
          } => {
              // println!("点击托盘切换窗口在最小和正常之间切换");
              let window = app.get_window("main").unwrap();
              if window.is_visible().unwrap() {
                  window.hide().unwrap();
              } else {
                  window.show().unwrap();
                  window.set_focus().unwrap();
              }

          }
          SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
          } => {
            println!("right click");
          }
          SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
          } => {
            println!("double click");
          }
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                std::process::exit(0);
              }
              "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
              }
              _ => {}
            }
          }
          _ => {}
        })
    .invoke_handler(tauri::generate_handler![custom_command, init_process,close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running application");
}
