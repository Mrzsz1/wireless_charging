use std::process::{Child, Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn configure_background_command(command: &mut Command) -> &mut Command {
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

pub fn configure_python_command(command: &mut Command) -> &mut Command {
    configure_background_command(command);
    command
        .env("PYTHONUTF8", "1")
        .env("PYTHONIOENCODING", "utf-8")
}

pub fn terminate_process_tree(child: &mut Child) {
    #[cfg(target_os = "windows")]
    {
        let mut command = Command::new("taskkill");
        configure_background_command(&mut command);
        let _ = command
            .args(["/PID", &child.id().to_string(), "/T", "/F"])
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn python_commands_force_utf8_output() {
        let mut command = Command::new("py");
        configure_python_command(&mut command);
        let envs = command.get_envs().collect::<Vec<_>>();
        assert!(envs.contains(&(OsStr::new("PYTHONUTF8"), Some(OsStr::new("1")))));
        assert!(envs.contains(&(OsStr::new("PYTHONIOENCODING"), Some(OsStr::new("utf-8")))));
    }
}
