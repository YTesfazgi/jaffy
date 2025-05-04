// Handles start/stop logic + global state

use std::process::{Command, Child}; // Configure and spawn external processes
use std::sync::{Arc, Mutex}; // Shared ownership of data across threads w/ mutual exclusion
use once_cell::sync::Lazy; // Initialize a static global value once

// Trait for process management to allow mocking
pub trait ProcessManager {
    fn spawn_process(&self) -> Result<(), String>;
    fn is_process_running(&self) -> bool;
    fn kill_process(&mut self) -> Result<(), String>;
}

// Trait for process operations
pub trait Process {
    fn kill(&mut self) -> Result<(), String>;
}

// Implementation for real Child process
struct RealProcess {
    child: Child,
}

impl RealProcess {
    fn new(child: Child) -> Self {
        RealProcess { child }
    }
}

impl Process for RealProcess {
    fn kill(&mut self) -> Result<(), String> {
        // First check if the process already exited
        if let Ok(Some(_)) = self.child.try_wait() {
            return Ok(());
        }
    
        #[cfg(unix)]
        {
            // Safe: `self.child.id()` returns a valid PID for a running process.
            unsafe {
                libc::kill(self.child.id() as i32, libc::SIGTERM);
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    
        #[cfg(windows)]
        {
            // Windows doesn't support Unix signals like SIGTERM.
            // A proper CTRL+C requires setting up a job object or using a crate like `ctrlc`.
            // For now, wait briefly to give the process a chance to exit.
            let _ = self.child.try_wait();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    
        // Force kill if still running
        match self.child.try_wait() {
            Ok(Some(_)) => Ok(()), // Process exited
            _ => self.child.kill().map_err(|e| format!("Failed to stop process: {}", e)),
        }
    }    
}

// Real implementation using FFmpeg
struct FFmpegManager {
    process: Mutex<Option<Box<dyn Process + Send>>>,
}

impl FFmpegManager {
    fn new() -> Self {
        FFmpegManager {
            process: Mutex::new(None),
        }
    }
    
    fn build_command() -> Command {
        let mut cmd = Command::new("ffmpeg");
        
        #[cfg(target_os = "macos")]
        {
            // macOS specific configuration using avfoundation
            // Format: "[[video device]:[audio device]]"
            // 1:0 typically means "screen:built-in microphone" on macOS
            cmd.args([
                "-f", "avfoundation",
                "-framerate", "15",
                "-i", "4", // Screen only
                "-pix_fmt", "yuv420p",
                "-c:v", "libx264",
                "-preset", "ultrafast",
                "-an", // no audio
                "-movflags", "+faststart",
                "output.mp4"
            ]);            
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows specific configuration using gdigrab
            cmd.args([
                "-f", "gdigrab", 
                "-framerate", "30", 
                "-i", "desktop", 
                "-c:v", "libx264",
                "-preset", "ultrafast",
                "-pix_fmt", "yuv420p",
                "-movflags", "+faststart",
                "output.mp4"
            ]);
        }
        
        cmd
    }
}

impl ProcessManager for FFmpegManager {
    fn spawn_process(&self) -> Result<(), String> {
        // Don't spawn if process is already running
        if self.is_process_running() {
            return Ok(());
        }
        
        let mut cmd = Self::build_command();
        let child = cmd.spawn()
            .map_err(|e| format!("Failed to start FFmpeg: {}", e))?;
            
        let process = Box::new(RealProcess::new(child));
        
        let mut handle = self.process.lock().unwrap();
        *handle = Some(process);
        
        Ok(())
    }
    
    fn is_process_running(&self) -> bool {
        self.process.lock().unwrap().is_some()
    }
    
    fn kill_process(&mut self) -> Result<(), String> {
        let mut handle = self.process.lock().unwrap();
        if let Some(mut process) = handle.take() {
            process.kill()?;
            Ok(())
        } else {
            Err("No process running.".into())
        }
    }
}

// Global instance for use in Tauri commands
static FFMPEG_MANAGER: Lazy<Arc<Mutex<FFmpegManager>>> = 
    Lazy::new(|| Arc::new(Mutex::new(FFmpegManager::new())));

#[allow(dead_code)]
#[tauri::command]
pub fn start_ffmpeg() -> Result<(), String> {
    let manager = FFMPEG_MANAGER.lock().unwrap();
    manager.spawn_process()
}

#[allow(dead_code)]
#[tauri::command]
pub fn stop_ffmpeg() -> Result<(), String> {
    let mut manager = FFMPEG_MANAGER.lock().unwrap();
    manager.kill_process()
}

// Return the status of the FFmpeg process
#[allow(dead_code)]
#[tauri::command]
pub fn ffmpeg_status() -> bool {
    let manager = FFMPEG_MANAGER.lock().unwrap();
    manager.is_process_running()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock implementation for testing
    #[allow(dead_code)]
    struct MockProcess {
        killed: bool,
    }
    
    #[allow(dead_code)]
    impl MockProcess {
        fn new() -> Self {
            MockProcess { killed: false }
        }
    }
    
    impl Process for MockProcess {
        fn kill(&mut self) -> Result<(), String> {
            self.killed = true;
            Ok(())
        }
    }
    
    struct MockProcessManager {
        process_running: bool,
    }
    
    impl MockProcessManager {
        fn new() -> Self {
            MockProcessManager {
                process_running: false,
            }
        }
    }
    
    impl ProcessManager for MockProcessManager {
        fn spawn_process(&self) -> Result<(), String> {
            // In a real implementation, we would set process_running here
            // but since it's immutable in this context, we just return Ok
            Ok(())
        }
        
        fn is_process_running(&self) -> bool {
            self.process_running
        }
        
        fn kill_process(&mut self) -> Result<(), String> {
            if self.process_running {
                self.process_running = false;
                Ok(())
            } else {
                Err("No process running.".into())
            }
        }
    }
    
    #[test]
    fn test_mock_process_manager() {
        let mut manager = MockProcessManager::new();
        
        // Initially no process is running
        assert_eq!(manager.is_process_running(), false);
        
        // Trying to kill when no process should fail
        assert!(manager.kill_process().is_err());
        
        // After spawning, process should be running
        let _ = manager.spawn_process();
        // In a real implementation with a non-mock, this would happen automatically
        manager.process_running = true; // Manually set for mock
        assert_eq!(manager.is_process_running(), true);
        
        // Should be able to kill
        assert!(manager.kill_process().is_ok());
        
        // After killing, no process should be running
        assert_eq!(manager.is_process_running(), false);
    }
}
