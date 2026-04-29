class Canon < Formula
  desc "__DESC__"
  homepage "__HOMEPAGE__"
  version "__VERSION__"
  license "__LICENSE__"

  on_macos do
    on_arm do
      url "__MACOS_ARM64_URL__"
      sha256 "__MACOS_ARM64_SHA256__"
    end

    on_intel do
      url "__MACOS_X86_64_URL__"
      sha256 "__MACOS_X86_64_SHA256__"
    end
  end

  on_linux do
    on_arm do
      url "__LINUX_ARM64_URL__"
      sha256 "__LINUX_ARM64_SHA256__"
    end

    on_intel do
      url "__LINUX_X86_64_URL__"
      sha256 "__LINUX_X86_64_SHA256__"
    end
  end

  def install
    bin.install "canon"
  end

  test do
    system bin/"canon", "init", "--output", "json"
    assert_path_exists testpath/".canon"
  end
end