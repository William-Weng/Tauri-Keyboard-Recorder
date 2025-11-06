use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::result::Result;
use std::thread::{spawn, sleep, JoinHandle};
use std::time::Duration;
use lazy_static::lazy_static;
use rdev::{listen, simulate, Event, EventType, Key, ListenError};
use rdev::EventType::{KeyPress, KeyRelease, MouseMove};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, HotKeyState};
use global_hotkey::hotkey::{HotKey, Code, Modifiers};

struct Recorder {
    events: Vec<Event>,
}

static IS_LISTENING: AtomicBool = AtomicBool::new(false);
static STOP_PLAYBACK: AtomicBool = AtomicBool::new(true);

// 用於存儲錄製事件的全局變數
lazy_static! {
    static ref RECORDER: Arc<Mutex<Recorder>> = Arc::new(Mutex::new(Recorder { events: Vec::new() }));
}

// 用於存儲執行緒 handles 的全局變數
lazy_static! {
    static ref THREAD_HANDLES: Arc<Mutex<Option<(std::thread::JoinHandle<Result<(), ListenError>>, std::thread::JoinHandle<()>)>>> = 
        Arc::new(Mutex::new(None));
}

// MARK: 處理各種事件的具體行為 
/// 處理鍵盤按下事件 (錄製)
/// # 參數
/// - `event` - Event
/// - `key` - Key
fn _key_press_action(event: &Event, key: Key) {
    record_event(event, key);
}

/// 處理鍵盤放開事件
/// # 參數
/// - `event` - Event
/// - `key` - Key
fn _key_release_action(_event: &Event, _key: Key) {}

/// 處理滑鼠移動事件
/// # 參數
/// - `event` - Event
/// - `x` - f64
/// - `y` - f64
fn _mouse_move_action(_event: &Event, _x: f64, _y: f64) {}

/// 模擬鍵盤按下事件
/// # 參數
/// - `key` - Key
fn _play_key_press(key: Key) {
    
    match simulate(&EventType::KeyPress(key)) {
        Ok(_) => println!("成功模擬按鍵: {:?}", key),
        Err(error) => println!("模擬鍵盤事件失敗: {:?}", error),
    }
}

/// 模擬鍵盤放開事件
/// # 參數
/// - `key` - Key
fn _play_key_release(key: Key) {
    
    match simulate(&EventType::KeyRelease(key)) {
        Ok(_) => println!("成功模擬放開按鍵: {:?}", key),
        Err(error) => println!("模擬鍵盤事件失敗: {:?}", error),
    }
}

/// 模擬滑鼠移動事件
/// # 參數
/// - `x` - f64
/// - `y` - f64
/// - `delay_time` - u64 延遲時間（毫秒）
fn _play_mouse_move(_x: f64, _y: f64) {}

/// 回調事件處理 (鍵盤 / 滑鼠)
/// # 參數
/// - `event` - Event
fn callback(event: Event) {

    match event.event_type {
        KeyPress(key) => { _key_press_action(&event, key); }
        KeyRelease(key) => { _key_release_action(&event, key); }
        MouseMove { x, y } => { _mouse_move_action(&event, x, y); }
        _ => {}
    }
}

// MARK: 相關功能實現
/// 開始監聽鍵盤和滑鼠事件 (新執行緒)
/// # 返回值
/// - JoinHandle<Result<(), ListenError>> - 執行緒的 handle，可用於檢查執行狀態
fn listen_keyboard() -> JoinHandle<Result<(), ListenError>> {

    spawn(move || {
        listen_keyboard_action().map_err(|error| { 
            println!("[Error] 鍵盤監聽執行緒發生錯誤: {:?}", error);
            error
        })
    })
}

/// 開始監聽鍵盤和滑鼠事件
/// # 返回值
/// - Result<(), ListenError>
fn listen_keyboard_action() -> Result<(), ListenError> {

    match listen(callback) {
        Ok(()) => Ok(()),
        Err(error) => { Err(error) },
    }

    // let result = listen(callback).map_err(|error| { error })
}

/// 記錄事件值
/// # 參數
/// - `event` - Event
/// - `key` - Key
fn record_event(event: &Event, _key: Key) {

    if !IS_LISTENING.load(Ordering::SeqCst) { return; }

    let mut recorder = RECORDER.lock().unwrap();
    recorder.events.push(event.clone());
}

/// 註冊全局熱鍵行為 (新執行緒)
/// # 返回值
/// - JoinHandle<()> - 執行緒的 handle，可用於檢查執行狀態
fn register_hotkey_action() -> JoinHandle<()> {

    spawn(move || {
        
        let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create GlobalHotKey manager");
        let record_hotkey = HotKey::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyG);  // Command + Shift + G => 開始錄製
        let stop_hotkey = HotKey::new(Some(Modifiers::META | Modifiers::CONTROL), Code::KeyG);  // Command + Control + G => 停止操作
        let playback_hotkey = HotKey::new(Some(Modifiers::META | Modifiers::ALT), Code::KeyG);  // Command + Option + G => 執行回放
        
        hotkey_manager.register(record_hotkey.clone()).expect("Failed to register Command+Shift+G hotkey");
        hotkey_manager.register(stop_hotkey.clone()).expect("Failed to register Command+Control+G hotkey");
        hotkey_manager.register(playback_hotkey.clone()).expect("Failed to register Command+Option+G hotkey");

        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().recv() {

                if event.state == HotKeyState::Pressed {
                    if event.id() == record_hotkey.id() { start_listen(); } 
                    else if event.id() == stop_hotkey.id() { stop_listen(); } 
                    else if event.id() == playback_hotkey.id() { playback(); }
                }
            } else {
                break;
            }
        }
    })
}

// MARK: 使用到的功能
/// 初始化設置
/// # 返回值
/// - Result<(JoinHandle<Result<(), ListenError>>, JoinHandle<()>), String>
fn init_setting() -> Result<(JoinHandle<Result<(), ListenError>>, JoinHandle<()>), String> {
    let keyboard_handle = listen_keyboard();
    let hotkey_handle = register_hotkey_action();
    Ok((keyboard_handle, hotkey_handle))
}

/// 開始監聽錄製鍵盤事件
fn start_listen() -> bool {
    STOP_PLAYBACK.store(false, Ordering::SeqCst);
    IS_LISTENING.store(true, Ordering::SeqCst);
    RECORDER.lock().unwrap().events.clear();

    IS_LISTENING.load(Ordering::SeqCst)
}

/// 停止監聽錄製鍵盤事件
fn stop_listen() -> bool {
    STOP_PLAYBACK.store(true, Ordering::SeqCst);
    IS_LISTENING.store(false, Ordering::SeqCst);

    IS_LISTENING.load(Ordering::SeqCst)
}

/// 檢查背景的鍵盤監聽執行緒（thread）是否還在執行中
/// # 返回值
/// - bool - 如果執行緒還在運行，返回 true；否則返回 false
fn check_keyboard_status() -> bool {
    if let Some((keyboard_handle, _)) = &*THREAD_HANDLES.lock().unwrap() { return !keyboard_handle.is_finished(); }
    return false;
    // THREAD_HANDLES.lock().unwrap()
    //     .as_ref()
    //     .map_or(false, |(keyboard_handle, _)| !keyboard_handle.is_finished())
}

/// 回放錄製的鍵盤事件
/// # 返回值
/// - bool - 如果回放過程中被停止，返回 true；否則返回 false
fn playback() -> bool {
    
    let events = RECORDER.lock().unwrap().events.clone();
    if events.is_empty() { return false; }

    let mut is_stop = false;

    sleep(Duration::from_millis(250));
    STOP_PLAYBACK.store(false, Ordering::SeqCst);

    let mut last_event_time = events[0].time;

    for event in events.iter() {
        
        if STOP_PLAYBACK.load(Ordering::SeqCst) { is_stop = true; break; }

        // 計算與上一個事件之間的時間差並等待
        let delay = event.time.duration_since(last_event_time).unwrap_or_default();
        sleep(delay);

        // 根據事件類型直接模擬，不再傳入延遲時間
        match event.event_type {
            KeyPress(key) => { _play_key_press(key); }
            KeyRelease(key) => {  _play_key_release(key); }
            MouseMove { x, y } => { _play_mouse_move(x, y); }
            _ => {}
        }

        last_event_time = event.time;
    }

    is_stop
}

// MARK: Tauri commands
#[tauri::command]
fn start_record() -> bool { start_listen() }

#[tauri::command]
fn stop_record() -> bool { stop_listen() }

#[tauri::command]
fn play_record() -> bool { playback() }

#[tauri::command]
fn keyboard_status() -> bool { check_keyboard_status() }

// MARK: Tauri app entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let handles = init_setting().expect("Failed to initialize settings");
    *THREAD_HANDLES.lock().unwrap() = Some(handles);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_record,
            stop_record,
            play_record,
            keyboard_status
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
