# Homebrew Formula for comfy-conv

class ComfyConv < Formula
  desc "Interactive CLI document converter with a beautiful TUI"
  homepage "https://github.com/JUSTMEETPATEL/Comfy-Conv"
  url "https://github.com/JUSTMEETPATEL/Comfy-Conv/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "4232009a70f703cba11d337ba6efa6fc2fc8c9b3370ae6d470c40d443416a6ba"
  license "MIT"

  depends_on "rust" => :build
  depends_on "pandoc"

  def install
    system "cargo", "install", *std_cargo_args
  end

  def caveats
    <<~EOS
      comfy-conv requires LibreOffice for Office document conversions.
      
      To install LibreOffice, run:
        brew install --cask libreoffice
      
      Or run the built-in setup:
        comfy-conv --setup
    EOS
  end

  test do
    assert_match "comfy-conv", shell_output("#{bin}/comfy-conv --version")
  end
end
