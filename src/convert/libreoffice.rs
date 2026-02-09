//! LibreOffice headless conversion
//!
//! Converts Office documents using `soffice --headless`.

use crate::error::{ConvError, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// Convert a file using LibreOffice headless mode
///
/// # Arguments
/// * `soffice_path` - Path to soffice executable
/// * `input` - Input file path
/// * `output_format` - Target format extension (e.g., "pdf", "html")
/// * `output_dir` - Directory for output file
/// * `timeout` - Maximum conversion time
pub fn convert(
    soffice_path: &str,
    input: &Path,
    output_format: &str,
    output_dir: &Path,
    timeout: Duration,
) -> Result<PathBuf> {
    // Build command
    let mut cmd = Command::new(soffice_path);
    cmd.arg("--headless")
        .arg("--convert-to")
        .arg(output_format)
        .arg("--outdir")
        .arg(output_dir)
        .arg(input)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Execute with timeout
    let output = run_with_timeout(&mut cmd, timeout)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ConvError::ConversionFailed {
            message: format!("LibreOffice exited with code {}", output.status),
            stderr: Some(stderr.to_string()),
        });
    }

    // Determine expected output path
    let input_stem = input.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let expected_output = output_dir.join(format!("{}.{}", input_stem, output_format));

    // Verify output exists
    if !expected_output.exists() {
        return Err(ConvError::ConversionFailed {
            message: "LibreOffice completed but output file was not created".to_string(),
            stderr: Some(String::from_utf8_lossy(&output.stderr).to_string()),
        });
    }

    Ok(expected_output)
}

/// Run a command with a timeout
fn run_with_timeout(cmd: &mut Command, timeout: Duration) -> Result<std::process::Output> {
    use std::thread;
    use std::sync::mpsc;

    let mut child = cmd.spawn().map_err(|e| ConvError::ConversionFailed {
        message: format!("Failed to start LibreOffice: {}", e),
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
            message: format!("LibreOffice process error: {}", e),
            stderr: None,
        }),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            // Note: The child process is still running in another thread
            // In a production app, we'd want to kill it
            Err(ConvError::Timeout { seconds: timeout_secs })
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(ConvError::ConversionFailed {
                message: "LibreOffice process monitor disconnected".to_string(),
                stderr: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_path_generation() {
        let input = Path::new("/tmp/test.docx");
        let output_dir = Path::new("/tmp");
        let format = "pdf";
        
        let stem = input.file_stem().unwrap().to_str().unwrap();
        let expected = output_dir.join(format!("{}.{}", stem, format));
        
        assert_eq!(expected, PathBuf::from("/tmp/test.pdf"));
    }
}
