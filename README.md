# comfy-conv

> 🔄 Interactive CLI document converter with a beautiful TUI

Convert documents between formats through an intuitive terminal interface. No more memorizing complex command-line flags!

![Demo](https://raw.githubusercontent.com/JUSTMEETPATEL/Comfy-Conv/main/demo.gif)

## ✨ Features

- 📁 **Visual file picker** - Navigate with arrow keys or vim-style (j/k)
- 🎨 **Beautiful TUI** - Modern terminal interface with colors
- 🔄 **Smart format detection** - Automatically shows compatible output formats
- ⚡ **Fast conversions** - Uses LibreOffice and Pandoc under the hood
- 🛠️ **Auto-setup** - Install dependencies with `--setup`

## 📦 Installation

### macOS (Homebrew)

```bash
brew tap justmeetpatel/tap
brew install comfy-conv
comfy-conv --setup  # Installs LibreOffice
```

### Windows

**Option 1: PowerShell (recommended)**
```powershell
irm https://raw.githubusercontent.com/JUSTMEETPATEL/Comfy-Conv/main/install.ps1 | iex
comfy-conv --setup
```

**Option 2: Manual**
1. Download `comfy-conv-windows-x64.exe` from [Releases](https://github.com/JUSTMEETPATEL/Comfy-Conv/releases)
2. Rename to `comfy-conv.exe` and add to PATH
3. Run `comfy-conv --setup`

### Linux

```bash
# Download binary
curl -L https://github.com/JUSTMEETPATEL/Comfy-Conv/releases/latest/download/comfy-conv-linux-x64 -o comfy-conv
chmod +x comfy-conv
sudo mv comfy-conv /usr/local/bin/

# Install dependencies
comfy-conv --setup
```

### From Source (Cargo)

```bash
cargo install --git https://github.com/JUSTMEETPATEL/Comfy-Conv.git
comfy-conv --setup
```

## 🚀 Usage

```bash
# Interactive mode - pick a file and convert
comfy-conv

# Install missing dependencies
comfy-conv --setup
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Select |
| `Esc` / `q` | Cancel/Quit |

## 📄 Supported Formats

| Input | Output Options |
|-------|----------------|
| DOCX | PDF, HTML, TXT, Markdown |
| XLSX | PDF, HTML, CSV |
| PPTX | PDF, HTML |
| Markdown | HTML, DOCX, TXT |
| HTML | Markdown, DOCX, TXT |
| TXT | HTML, DOCX, Markdown |

## 🔧 Dependencies

comfy-conv uses these tools for conversions:

- **LibreOffice** - Office documents (DOCX, XLSX, PPTX → PDF)
- **Pandoc** - Text formats (Markdown, HTML, TXT)

Run `comfy-conv --setup` to install them automatically.

## 📝 License

MIT © [Meet Patel](https://github.com/JUSTMEETPATEL)
