//! Pandoc conversion
//!
//! Converts text documents using Pandoc.

use crate::error::{ConvError, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Convert a file using Pandoc
///
/// # Arguments
/// * `pandoc_path` - Path to pandoc executable
/// * `input` - Input file path
/// * `output` - Output file path
/// * `timeout` - Maximum conversion time
pub fn convert(
    pandoc_path: &str,
    input: &Path,
    output: &Path,
    timeout: Duration,
) -> Result<()> {

    // Build command
    let mut cmd = Command::new(pandoc_path);
    cmd.arg(input)
        .arg("-o")
        .arg(output)
        .arg("--standalone")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Execute with timeout
    let output_result = run_with_timeout(&mut cmd, timeout)?;

    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(ConvError::ConversionFailed {
            message: format!("Pandoc exited with code {}", output_result.status),
            stderr: Some(stderr.to_string()),
        });
    }

    // Verify output exists
    if !output.exists() {
        return Err(ConvError::ConversionFailed {
            message: "Pandoc completed but output file was not created".to_string(),
            stderr: Some(String::from_utf8_lossy(&output_result.stderr).to_string()),
        });
    }

    Ok(())
}

/// Check if LaTeX is available for PDF generation
fn has_latex() -> bool {
    // Try common LaTeX commands
    for cmd in &["pdflatex", "xelatex", "lualatex"] {
        if Command::new(cmd)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return true;
        }
    }
    false
}

/// Run a command with a timeout
fn run_with_timeout(cmd: &mut Command, timeout: Duration) -> Result<std::process::Output> {
    use std::thread;
    use std::sync::mpsc;

    let mut child = cmd.spawn().map_err(|e| ConvError::ConversionFailed {
        message: format!("Failed to start Pandoc: {}", e),
        stderr: None,
    })?;

    let (tx, rx) = mpsc::channel();
    let timeout_secs = timeout.as_secs();

    // Spawn thread to wait for child
    thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    // Wait with timeout
    match rx.recv_timeout(timeout) {
        Ok(result) => result.map_err(|e| ConvError::ConversionFailed {
            message: format!("Pandoc process error: {}", e),
            stderr: None,
        }),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            Err(ConvError::Timeout { seconds: timeout_secs })
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(ConvError::ConversionFailed {
                message: "Pandoc process monitor disconnected".to_string(),
                stderr: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_check() {
        // This test just verifies the function doesn't panic
        let _ = has_latex();
    }
}
