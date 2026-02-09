# Product Requirements Document: comfy-conv

**Version:** 1.0

**Last Updated:** February 9, 2026

**Status:** MVP Specification

---

## Executive Summary

**comfy-conv** is an interactive command-line tool that converts documents between common formats (DOCX, XLSX, PPTX, Markdown, PDF, HTML, TXT) through a modern, colorful, arrow-key–driven terminal interface. Built in Rust for performance and correctness, it prioritizes developer experience with a clean Next.js–style interaction model while handling the complexity of format conversion through LibreOffice and Pandoc.

### Vision

Create the most delightful document conversion experience in the terminal—where converting a PowerPoint to PDF feels as smooth as navigating a modern web application.

### Success Metrics

* **Conversion accuracy:** 100% successful conversions for supported format pairs
* **Performance:** <3 seconds for typical document conversions (excluding LibreOffice startup)
* **User satisfaction:** Zero-friction interactive flow requiring minimal keystrokes
* **Error clarity:** All failures explained with actionable next steps

---

## Product Overview

### Problem Statement

Existing document conversion tools suffer from multiple pain points:

1. **Poor UX:** Command-line tools require memorizing syntax and format flags
2. **Batch complexity:** Most tools over-engineer for batch processing, adding cognitive load
3. **Hidden dependencies:** Users discover missing LibreOffice or Pandoc only after errors
4. **Format confusion:** No clear guidance on which conversions are supported
5. **Silent failures:** Cryptic errors with no actionable guidance

### Target Users

**Primary Persona: The Productive Developer**

* Frequently converts documents for documentation, reports, presentations
* Comfortable in the terminal but values aesthetics and UX
* Expects tools to be fast, reliable, and self-explanatory
* Wants automation without configuration overhead

**Secondary Persona: The Command-Line Enthusiast**

* Appreciates modern CLI tools (ripgrep, fd, bat)
* Values interactive experiences over flag memorization
* Willing to install dependencies if the tool is excellent

### Solution

An interactive CLI that:

1. **Discovers** available files in the current directory
2. **Detects** input format automatically
3. **Suggests** compatible output formats
4. **Converts** using the best engine (LibreOffice or Pandoc)
5. **Saves** output alongside the original with clear feedback

All through an arrow-key interface styled like modern TUIs (Terminal User Interfaces).

---

## Core Features (MVP Scope)

### 1. Interactive File Selection

**Behavior:**

* When launched without arguments (`comfy-conv`), display a file picker
* List all convertible files in the current directory (recursive optional for v2)
* Show file type icons/indicators (📄 DOCX, 📊 XLSX, 📽️ PPTX, 📝 MD)
* Support arrow-key navigation, type-to-filter, Enter to select
* Display file size and last modified date

**Technical Requirements:**

* Use `crossterm` or `ratatui` for TUI rendering
* Support both arrow keys and vim-style navigation (j/k)
* Filter files by extension: `.docx`, `.xlsx`, `.pptx`, `.md`, `.pdf`, `.html`, `.txt`
* Handle empty directories gracefully with helpful message

**Example Interface:**

```
╭─ Select a file to convert ────────────────────────────────╮
│                                                            │
│  > 📄 proposal.docx              124 KB   2 hours ago     │
│    📊 budget.xlsx                 45 KB   3 days ago      │
│    📽️ deck.pptx                  2.3 MB   1 week ago      │
│    📝 README.md                   12 KB   today           │
│                                                            │
│  Use ↑↓ to navigate, Enter to select, Esc to cancel       │
╰────────────────────────────────────────────────────────────╯
```

### 2. Smart Format Detection

**Behavior:**

* Automatically detect input format from file extension
* Validate file integrity (is it actually a DOCX or corrupted?)
* Display detected format prominently in the UI

**Supported Input Formats:**

* **Office documents:** `.docx`, `.xlsx`, `.pptx`
* **Markdown:** `.md`
* **Web formats:** `.html`
* **Plain text:** `.txt`
* **PDF:** `.pdf` (limited—only for PDF→TXT/HTML via Pandoc)

**Technical Requirements:**

* Extension-based detection (fast, reliable)
* Optional magic number validation for Office formats (ZIP signature)
* Clear error if file is corrupted or unsupported

### 3. Target Format Selection

**Behavior:**

* Show only compatible output formats for the selected input
* Display format descriptions (e.g., "PDF - Portable Document Format, best for sharing")
* Arrow-key selection with instant preview of output filename
* Smart defaults (e.g., DOCX → PDF is highlighted first)

**Format Compatibility Matrix:**

| Input     | Output Formats       | Conversion Engine        |
| --------- | -------------------- | ------------------------ |
| `.docx` | PDF, HTML, TXT, MD   | LibreOffice + Pandoc     |
| `.xlsx` | PDF, HTML, CSV       | LibreOffice              |
| `.pptx` | PDF, HTML            | LibreOffice              |
| `.md`   | DOCX, PDF, HTML, TXT | Pandoc                   |
| `.html` | DOCX, PDF, TXT, MD   | Pandoc                   |
| `.txt`  | DOCX, PDF, HTML, MD  | Pandoc                   |
| `.pdf`  | TXT, HTML            | Pandoc (text extraction) |

**Technical Requirements:**

* Hardcoded compatibility matrix (no dynamic discovery for MVP)
* Display recommended format first (e.g., DOCX→PDF, MD→HTML)
* Show conversion engine used (LibreOffice vs Pandoc) for transparency

**Example Interface:**

```
╭─ Convert proposal.docx to: ───────────────────────────────╮
│                                                            │
│  > 📕 PDF      Portable Document Format (LibreOffice)     │
│    🌐 HTML     Web page (Pandoc)                           │
│    📝 TXT      Plain text (Pandoc)                         │
│    📋 MD       Markdown (Pandoc)                           │
│                                                            │
│  Output: proposal.pdf                                      │
│  Use ↑↓ to navigate, Enter to convert, Esc to go back     │
╰────────────────────────────────────────────────────────────╯
```

### 4. Conversion Execution

**Behavior:**

* Show confirmation screen with input → output summary
* Display a progress spinner during conversion
* Provide real-time status updates ("Starting LibreOffice...", "Converting...")
* Handle errors gracefully with actionable messages
* Show success message with output file path

**Progress States:**

1. **Preparing:** Validating input, checking dependencies
2. **Converting:** Running LibreOffice/Pandoc
3. **Finalizing:** Writing output file
4. **Complete:** Display success/failure

**Technical Requirements:**

* Async conversion with spinner animation (using `indicatif` or similar)
* Timeout handling (30s for LibreOffice, 10s for Pandoc)
* Capture stderr from LibreOffice/Pandoc for debugging
* Atomic file writes (write to temp, then rename)
* Preserve original file timestamps where possible

**Example Interface (Converting):**

```
╭─ Converting ──────────────────────────────────────────────╮
│                                                            │
│  📄 proposal.docx  →  📕 proposal.pdf                      │
│                                                            │
│  ⠋ Converting with LibreOffice...                         │
│                                                            │
│  This may take a few seconds                              │
╰────────────────────────────────────────────────────────────╯
```

**Example Interface (Success):**

```
╭─ Conversion Complete ─────────────────────────────────────╮
│                                                            │
│  ✓ Successfully converted proposal.docx to PDF            │
│                                                            │
│  📕 proposal.pdf (234 KB)                                  │
│  📂 /Users/dev/documents/proposal.pdf                      │
│                                                            │
│  Press Enter to convert another file, or Esc to exit      │
╰────────────────────────────────────────────────────────────╯
```

### 5. Output File Management

**Behavior:**

* Save converted file in the same directory as the input
* Use naming pattern: `{original_name}.{new_extension}`
* Handle filename collisions gracefully (prompt to overwrite or auto-rename)
* Show full output path after successful conversion

**Filename Collision Handling:**

* **Option 1 (MVP):** Prompt user: Overwrite / Rename / Cancel
* **Option 2 (Future):** Auto-append timestamp `proposal-2026-02-09.pdf`

**Technical Requirements:**

* Check for existing files before conversion
* Preserve input file metadata (timestamps, permissions)
* Use OS-specific path separators
* Handle long filenames (truncate if >255 chars)

**Example Collision Prompt:**

```
╭─ File Already Exists ─────────────────────────────────────╮
│                                                            │
│  ⚠️  proposal.pdf already exists (234 KB)                 │
│                                                            │
│  > Overwrite                                               │
│    Rename to proposal-1.pdf                                │
│    Cancel                                                  │
│                                                            │
│  Use ↑↓ to navigate, Enter to select, Esc to cancel       │
╰────────────────────────────────────────────────────────────╯
```

---

## Non-Goals (Out of Scope for MVP)

### Explicitly Excluded Features

1. **Batch conversion:** No multi-file processing in MVP
   * *Rationale:* Adds complexity to UX and error handling; single-file flow is more testable
2. **Configuration files:** No `.comfy-conv.toml` or global settings
   * *Rationale:* Interactive flow eliminates need for defaults; reduces maintenance burden
3. **Format-specific options:** No quality settings, page size, DPI, etc.
   * *Rationale:* Uses engine defaults; 80% use case doesn't need customization
4. **Custom output paths:** Output always goes next to input file
   * *Rationale:* Predictable behavior; reduces cognitive load
5. **GUI version:** Terminal-only for MVP
   * *Rationale:* Focus on CLI-first experience; GUI is a separate product
6. **Cloud storage integration:** No Google Drive, Dropbox, etc.
   * *Rationale:* Local files only; keeps scope manageable
7. **Format previews:** No rendering of converted output in terminal
   * *Rationale:* Complex and unreliable; users can open files externally
8. **Undo/history:** No conversion history or rollback
   * *Rationale:* Conversions are non-destructive; original files preserved

---

## Technical Architecture

### Technology Stack

**Language:** Rust (2021 edition)

* **Justification:** Memory safety, performance, excellent CLI ecosystem

**Key Dependencies:**

* **TUI framework:** `ratatui` (formerly tui-rs) for interactive interface
* **Terminal control:** `crossterm` for cross-platform terminal manipulation
* **Progress indicators:** `indicatif` for spinners and progress bars
* **Argument parsing:** `clap` (v4) for future CLI mode
* **File system:** `std::fs` + `walkdir` for file discovery
* **Process execution:** `tokio::process` or `std::process::Command` for LibreOffice/Pandoc
* **Error handling:** `anyhow` for error context, `thiserror` for custom error types

**External Tools:**

* **LibreOffice:** Headless mode for Office document conversions
* **Pandoc:** Universal document converter for text-based formats

### System Requirements

**Minimum:**

* **OS:** Linux, macOS, Windows 10+
* **Rust:** 1.70+ (for building from source)
* **Dependencies:**
  * LibreOffice 7.0+ (headless mode)
  * Pandoc 2.19+
* **Disk:** 10 MB for binary, 500 MB for LibreOffice, 100 MB for Pandoc
* **RAM:** 256 MB minimum, 1 GB recommended (LibreOffice can be memory-hungry)

**Recommended:**

* Modern terminal emulator supporting Unicode and colors (iTerm2, Alacritty, Windows Terminal)
* Fast SSD for large document conversions

### Dependency Detection

**Startup Check:**

```rust
// Pseudo-code
fn check_dependencies() -> Result<DependencyStatus> {
    let libreoffice = which("soffice") || which("libreoffice");
    let pandoc = which("pandoc");
  
    if !libreoffice.is_ok() {
        warn!("LibreOffice not found - Office document conversions disabled");
    }
    if !pandoc.is_ok() {
        warn!("Pandoc not found - text format conversions disabled");
    }
  
    if !libreoffice.is_ok() && !pandoc.is_ok() {
        return Err("No conversion engines found. Install LibreOffice or Pandoc.");
    }
  
    Ok(DependencyStatus { libreoffice, pandoc })
}
```

**Graceful Degradation:**

* If LibreOffice missing: Hide Office format conversions from UI
* If Pandoc missing: Hide text-based conversions from UI
* Show installation instructions in error messages (OS-specific)

### Conversion Engines

#### LibreOffice (Headless Mode)

**Usage:**

```bash
soffice --headless --convert-to pdf --outdir /path/to/output input.docx
```

**Implementation Details:**

* Spawn as child process with timeout (30s default)
* Capture stdout/stderr for error diagnostics
* LibreOffice auto-detects input format
* Use `--infilter` for ambiguous formats (optional)
* Output file appears in `--outdir` with same basename

**Error Handling:**

* **Timeout:** Kill process, inform user conversion took too long
* **Exit code ≠ 0:** Parse stderr for error message
* **Output file missing:** Treat as conversion failure even if exit code is 0

**Supported Conversions:**

* DOCX/XLSX/PPTX → PDF, HTML
* XLSX → CSV (via `--convert-to csv`)

#### Pandoc

**Usage:**

```bash
pandoc input.md -o output.pdf
pandoc input.html -o output.docx
```

**Implementation Details:**

* Spawn as child process with timeout (10s default)
* Auto-detects input format from extension
* Output format from `-o` extension
* Use `--standalone` for complete HTML/PDF output
* PDF generation requires LaTeX (future: detect and warn)

**Error Handling:**

* **Timeout:** Kill process, report timeout
* **Exit code ≠ 0:** Parse stderr for error (usually self-explanatory)
* **PDF without LaTeX:** Suggest installing LaTeX or converting to HTML instead

**Supported Conversions:**

* MD ↔ DOCX, HTML, PDF, TXT
* HTML ↔ MD, DOCX, TXT
* TXT → DOCX, HTML, PDF, MD
* PDF → TXT (text extraction)

### Error Handling Strategy

**Error Categories:**

1. **Dependency Missing:**
   * **Message:** "LibreOffice not found. Install it to convert Office documents."
   * **Action:** Show OS-specific installation command
   * **Example:** `brew install libreoffice` (macOS)
2. **Unsupported Conversion:**
   * **Message:** "Cannot convert PPTX to XLSX - incompatible formats."
   * **Action:** Show supported output formats for PPTX
3. **Conversion Failure:**
   * **Message:** "LibreOffice failed: Document is password-protected."
   * **Action:** Display stderr output from LibreOffice
   * **Guidance:** "Remove password protection or try a different file."
4. **File System Errors:**
   * **Message:** "Cannot write to /readonly/dir - permission denied."
   * **Action:** Check write permissions, suggest alternative location
5. **Timeout:**
   * **Message:** "Conversion timed out after 30 seconds."
   * **Action:** Suggest trying a smaller file or increasing timeout (future)

**Error Display:**

```
╭─ Conversion Failed ───────────────────────────────────────╮
│                                                            │
│  ✗ LibreOffice failed to convert proposal.docx            │
│                                                            │
│  Error: Document is password-protected                    │
│                                                            │
│  Suggestions:                                              │
│  • Remove password protection in Word                     │
│  • Try converting a different file                        │
│  • Check that the file isn't corrupted                    │
│                                                            │
│  Press Enter to try another file, or Esc to exit          │
╰────────────────────────────────────────────────────────────╯
```

### Testing Strategy

**Unit Tests:**

* File extension detection
* Format compatibility matrix lookup
* Filename collision handling
* Path sanitization

**Integration Tests:**

* LibreOffice conversion (requires LibreOffice installed)
* Pandoc conversion (requires Pandoc installed)
* Error handling for missing dependencies
* Timeout behavior

**Manual Testing Checklist:**

* [ ] File picker displays all supported formats
* [ ] Arrow keys navigate smoothly
* [ ] Type-to-filter works
* [ ] Format selection shows correct compatibility
* [ ] Spinner animates during conversion
* [ ] Success message shows correct path
* [ ] Error messages are actionable
* [ ] File collisions are handled gracefully
* [ ] Works on Linux, macOS, Windows

**Test Fixtures:**

* Sample DOCX, XLSX, PPTX files (5-50 KB)
* Sample Markdown with various features
* Corrupted files (invalid ZIP, wrong extension)
* Large files (10+ MB) for timeout testing

---

## User Experience Design

### Color Palette (Inspired by Next.js CLI)

* **Primary (Accent):** Cyan `#00D9FF` for selection, highlights
* **Success:** Green `#00FF00` for checkmarks, success messages
* **Error:** Red `#FF4444` for errors, warnings
* **Muted:** Gray `#888888` for secondary text, hints
* **Background:** Terminal default (respect user's theme)
* **Borders:** Subtle gray `#444444` for UI frames

### Typography

* **Headings:** Bold white
* **Body text:** Normal white
* **File names:** Bold cyan (emphasize important info)
* **File metadata:** Muted gray (size, date)
* **Hints:** Italic gray

### Interaction Patterns

**Navigation:**

* `↑/↓` or `j/k`: Move selection up/down
* `Enter`: Confirm selection
* `Esc`: Go back or exit
* `/` or `Ctrl+F`: Filter/search (future)
* `q`: Quick exit (exit immediately)

**Accessibility:**

* Support screen readers (output plain text mode with `--no-color` flag)
* High contrast mode (future: `--high-contrast`)
* Keyboard-only navigation (no mouse required)

### Animation & Feedback

**Spinner Frames (using indicatif):**

```
⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
```

**Transitions:**

* Smooth fade between screens (if terminal supports)
* Instant response to keystrokes (no lag)

**Progress Indicator:**

* Indeterminate spinner for unknown duration
* Estimated time remaining (future: based on file size)

---

## Deployment & Distribution

### Build & Release

**Build Process:**

```bash
# Development build
cargo build --release

# Platform-specific builds
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

**Binary Naming:**

* `comfy-conv` (Linux/macOS)
* `comfy-conv.exe` (Windows)

**Distribution Channels:**

1. **GitHub Releases:**
   * Pre-built binaries for Linux (x86_64), macOS (Intel + Apple Silicon), Windows (x64)
   * Source tarball
   * Checksums (SHA256)
2. **Package Managers (Future):**
   * **Homebrew (macOS/Linux):** `brew install comfy-conv`
   * **Cargo (Rust):** `cargo install comfy-conv`
   * **Scoop (Windows):** `scoop install comfy-conv`
   * **APT/DNF (Linux):** `.deb` and `.rpm` packages
3. **Installation Script (Future):**
   ```bash
   curl -sSL https://comfy-conv.sh | bash
   ```

### Installation Instructions

**From Binary:**

```bash
# Linux/macOS
curl -L https://github.com/user/comfy-conv/releases/latest/download/comfy-conv-linux -o comfy-conv
chmod +x comfy-conv
sudo mv comfy-conv /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/user/comfy-conv/releases/latest/download/comfy-conv-windows.exe" -OutFile "comfy-conv.exe"
Move-Item comfy-conv.exe C:\Windows\System32\
```

**From Source:**

```bash
git clone https://github.com/user/comfy-conv.git
cd comfy-conv
cargo install --path .
```

### Dependency Installation Guidance

**Embedded in Error Messages:**

```
LibreOffice not found.

Install it with:
  macOS:    brew install libreoffice
  Linux:    sudo apt install libreoffice  # Debian/Ubuntu
            sudo dnf install libreoffice  # Fedora
  Windows:  Download from https://www.libreoffice.org/

Pandoc not found.

Install it with:
  macOS:    brew install pandoc
  Linux:    sudo apt install pandoc
  Windows:  choco install pandoc
```

---

## Success Criteria & Metrics

### MVP Launch Criteria

**Functional Requirements:**

* ✅ Successfully converts between all supported format pairs
* ✅ Interactive UI works on Linux, macOS, Windows
* ✅ Dependency detection and graceful degradation
* ✅ Error messages are actionable and OS-specific
* ✅ Preserves original files (non-destructive)
* ✅ File collision handling works correctly

**Performance Requirements:**

* ✅ File picker renders in <100ms for 1000 files
* ✅ Format selection instantaneous (<50ms)
* ✅ Conversion completes in <5s for typical documents (500 KB)
* ✅ Binary size <10 MB (release build, stripped)
* ✅ Memory usage <50 MB during idle, <200 MB during conversion

**Quality Requirements:**

* ✅ Zero crashes on valid input files
* ✅ 100% test coverage for core conversion logic
* ✅ Documented code (rustdoc comments)
* ✅ CI/CD pipeline (GitHub Actions)

### Post-Launch Metrics

**Usage Metrics:**

* Number of conversions per week
* Most popular format pairs (DOCX→PDF likely #1)
* Conversion success rate (target: >95%)
* Error types and frequency

**Performance Metrics:**

* Average conversion time by format
* P50, P95, P99 latency
* Memory usage distribution

**User Satisfaction:**

* GitHub stars/forks (proxy for adoption)
* Issue open/close rate (responsiveness)
* Feature requests (product-market fit)

---

## Future Enhancements (Post-MVP)

### High Priority (v1.1)

1. **Batch Conversion:**

   * Select multiple files
   * Progress bar showing `[2/5] Converting...`
   * Aggregate success/failure report
2. **CLI Mode (Non-Interactive):**

   ```bash
   comfy-conv proposal.docx --to pdf
   comfy-conv *.md --to html --outdir ./web
   ```
3. **Conversion Presets:**

   * "Print" preset: DOCX→PDF with A4, 300 DPI
   * "Web" preset: DOCX→HTML with embedded images
   * "Archive" preset: DOCX→PDF/A
4. **Watch Mode:**

   ```bash
   comfy-conv --watch --from md --to html
   ```

   Auto-converts Markdown files on save (great for docs)

### Medium Priority (v1.2-1.3)

5. **Format-Specific Options:**
   * PDF: Page size, DPI, compression
   * HTML: Standalone vs fragment, CSS theme
   * XLSX: Delimiter for CSV export
6. **Custom Output Paths:**
   * `--outdir` flag for batch conversions
   * Template variables: `{name}.{ext}`, `{date}-{name}.pdf`
7. **Conversion History:**
   * Local SQLite database
   * View past conversions
   * Re-run previous conversion with one keystroke
8. **Improved Dependency Management:**
   * Auto-install LibreOffice/Pandoc (via package managers)
   * Bundled portable LibreOffice (macOS/Windows)
   * Fallback to cloud conversion API if local tools missing

### Low Priority (v2.0+)

9. **Plugin System:**
   * Custom conversion engines (ImageMagick, FFmpeg)
   * User-defined format mappings
   * Pre/post-processing hooks
10. **Cloud Integration:**
    * Convert files from Google Drive/Dropbox URLs
    * Upload results directly to cloud storage
    * Shareable conversion links
11. **GUI Version:**
    * Electron or Tauri desktop app
    * Drag-and-drop file conversion
    * Batch queue management
12. **Advanced Features:**
    * Document comparison (diff DOCX files)
    * Format linting (check Markdown/HTML validity)
    * Optical Character Recognition (OCR for scanned PDFs)

---

## Open Questions & Risks

### Open Questions

1. **PDF Generation from Markdown:**
   * Pandoc requires LaTeX for PDF output
   * Should we bundle LaTeX or suggest `wkhtmltopdf` alternative?
   * **Decision:** Detect LaTeX; if missing, offer HTML→PDF via LibreOffice as fallback
2. **File Size Limits:**
   * LibreOffice can hang on very large files (100+ MB)
   * Should we warn or block conversions above a threshold?
   * **Decision:** No hard limit for MVP; timeout handles hangs
3. **Filename Sanitization:**
   * How to handle special characters in filenames (spaces, Unicode)?
   * **Decision:** Preserve original filenames; rely on OS handling
4. **Cross-Platform Testing:**
   * Need CI runners for Linux, macOS, Windows
   * **Decision:** GitHub Actions provides free runners for all platforms
5. **License for Distribution:**
   * LibreOffice and Pandoc have permissive licenses (MPL, GPL)
   * Safe to distribute binaries that depend on them?
   * **Decision:** comfy-conv doesn't bundle them, just calls installed versions—no license conflict

### Risks & Mitigation

| Risk                                            | Impact | Probability | Mitigation                                                 |
| ----------------------------------------------- | ------ | ----------- | ---------------------------------------------------------- |
| LibreOffice API changes break compatibility     | High   | Low         | Pin to LTS versions, test on multiple LibreOffice versions |
| Pandoc deprecates formats (e.g., DOCX)          | Medium | Low         | Monitor Pandoc releases, maintain fallback to LibreOffice  |
| Large file conversions cause OOM crashes        | Medium | Medium      | Implement memory monitoring, timeout, warn users           |
| Terminal compatibility issues (colors, Unicode) | Low    | Medium      | Graceful fallback to plain ASCII,`--no-color`flag        |
| Windows LibreOffice headless mode unreliable    | High   | Medium      | Extensive Windows testing, document known issues           |
| Rust binary size bloat (>20 MB)                 | Low    | Low         | Strip symbols, optimize dependencies, use `cargo-bloat`  |

---

## Appendix

### Glossary

* **DXA:** One-twentieth of a point (1/1440 inch), used in DOCX dimensions
* **EMU:** English Metric Unit (1/914,400 inch), used in DOCX images
* **Headless mode:** Running LibreOffice without a graphical interface
* **TUI:** Terminal User Interface, interactive CLI apps (like `htop`)
* **Spinner:** Animated progress indicator for indeterminate tasks

### References

* [LibreOffice Headless Documentation](https://help.libreoffice.org/latest/en-US/text/shared/guide/start_parameters.html)
* [Pandoc User&#39;s Guide](https://pandoc.org/MANUAL.html)
* [Ratatui TUI Framework](https://ratatui.rs/)
* [The Rust CLI Book](https://rust-cli.github.io/book/)

### Version History

| Version | Date       | Changes                   |
| ------- | ---------- | ------------------------- |
| 1.0     | 2026-02-09 | Initial MVP specification |

---

**Document Owner:** Product Team

**Reviewers:** Engineering, Design

**Next Review:** Post-MVP launch (Q2 2026)
