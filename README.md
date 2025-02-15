# 🌌 Galactic Markdown

> In the neon-lit depths of your terminal, where bits dance and electrons dream, there exists a markdown editor so hoopy that even Ford Prefect would've added it to his Guide. Welcome to the digital frontier of document editing, you magnificent frood! 🚀

## 🎯 Why Another Markdown Editor?

Because in a universe of infinite improbability, we needed something improbably efficient. Don't Panic - this isn't just another editor. It's your towel in the digital age. Your markdown materializes in real-time, faster than a Pan Galactic Gargle Blaster hits your system. No flashy distractions, just you and your digital thoughts in a clean, cyberpunk-inspired interface.

We built this beauty with Rust because when you're hitchhiking through the galaxy, every CPU cycle counts. Navigate your documents like a digital nomad with seamless file operations. And for you code warriors out there, we've packed it with syntax highlighting that would make a Babel fish jealous.

## 🛠️ Technical Specs (For Nerds Like Us)

Deep in the quantum fabric of this editor lies a carefully crafted architecture that would make even Deep Thought's circuits tingle. We've engineered this beauty with more attention to detail than the Magratheans put into Earth Mk1.

### 🔧 Core Architecture

At the heart of our digital beast lies a state management system so elegant, it makes the Total Perspective Vortex look simple:

```rust
// The heart of our digital beast
struct GalacticMarkdown {
    markdown_input: String,        // Your thoughts, digitized
    commonmark_cache: CommonMarkCache,  // Quantum-speed rendering cache
    current_file: Option<PathBuf>, // Schrödinger's file - it may or may not exist
    unsaved_changes: bool,         // The uncertainty principle of document states
}
```

This isn't just any ordinary struct – it's a carefully balanced ecosystem where each field plays a crucial role:

- `markdown_input` handles your document content with Rust's memory-safe string type
- `commonmark_cache` powers the real-time preview rendering
- `current_file` uses Rust's `Option` type for bulletproof null-safety
- `unsaved_changes` tracks document state with atomic precision

### 🎯 Dependencies

Our dependency graph is leaner than a Bugblatter Beast of Traal on a diet:

```toml
[dependencies]
# The GUI Framework - More efficient than the Infinite Improbability Drive
eframe = "0.25.0"      # Modern window management with native GPU support
egui = "0.25.0"        # Immediate mode rendering with automatic DPI scaling

# Markdown Magic - Processes text faster than the Restaurant at the End of the Universe can prepare meals
egui_commonmark = "0.11.0" # CommonMark-compliant rendering with GFM support

# File Operations - More reliable than Eddie the Shipboard Computer
rfd = "0.12"           # Native file dialogs for your local dimension
```

### 🔮 Technical Features

The editor implements several algorithms that would impress even the mice who designed Earth:

1. **Document Management**
   - Native file system integration
   - Unsaved changes detection
   - File state persistence

2. **Rendering Pipeline**
   - Real-time markdown preview
   - CommonMark-compliant parsing
   - Syntax highlighting for code blocks

3. **User Interface**
   - Split-pane layout with adjustable preview
   - Native system dialogs
   - Dark mode support

### 🛡️ Safety Features

Because we care about your documents more than a Babel fish cares about brain waves:

- Full UTF-8 support (handles alien languages better than the fish itself)
- Rust's ownership-based memory safety
- Automatic file operation error handling
- Native backup prompts for unsaved changes

## 🚀 Quick Start

First, ensure you've got the following installed on your system:
- Rust (if not, `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Task (install from [taskfile.dev](https://taskfile.dev))

Then, plug into our dimension:

```bash
# Clone this repository into your local dimension
git clone https://github.com/neontowel/galactic-markdown.git
cd galactic-markdown

# Install development dependencies
task setup

# Launch into the digital frontier
task run
```

### 🎲 Development Commands

Our `Taskfile.yml` packs more punch than a Pan Galactic Gargle Blaster. Here's your guide to the galaxy:

```bash
task                 # View all available commands
task run            # Launch the editor
task watch          # Auto-rebuild on changes (perfect for development)
task build-release  # Build an optimized binary
task lint           # Check your code style
task test           # Run the test suite
task all            # Run all checks (lint, test, build)
```

Want to tweak the build? Pass custom flags through the CARGO_FLAGS variable:
```bash
task run -- CARGO_FLAGS="--features experimental"
```

## 🎮 Into the Digital Frontier

Your journey begins at the sleek, split-screen interface. Create new documents with `File > New`, or open existing markdown files through `File > Open`. Your work auto-saves to the digital aether with `File > Save`, or choose your own destination with `File > Save As`. Watch your markdown transform in real-time on the right panel while you type. Keep an eye out for the `*` indicator – your trusty companion warning you of unsaved changes.

## 🌟 The Digital Canvas

The editor comes packed with everything a digital scribe needs. Your code snippets shine in monospace glory. Tables, lists, headers, and code blocks spring to life in the preview. Links and images render instantly, all styled with native egui aesthetics. The preview updates in real-time, because waiting for refreshes is so last century.

## 🛸 Under the Hood

We've assembled this digital marvel using only the finest components in the galaxy. The blazing-fast egui/eframe powers our GUI, while egui_commonmark transforms your markdown with the precision of a laser cutter. Native file dialogs feel right at home on your system, and it's all wrapped in 100% Pure Rust for that sweet, sweet performance boost.

## 🌌 Join the Rebellion

Found a bug in the matrix? Want to add a feature that would make Deep Thought proud? Pull requests are welcome! Just remember to keep it simple – like the Answer to Life, the Universe, and Everything. Test your changes (we don't want any Vogons breaking the build), and document your code to help other froods understand your genius.

## ⚖️ License

MIT - Because sharing is caring, even at the end of the universe.

---

> "Oh no, not again." - The Bowl of Petunias, upon realizing they hadn't backed up their markdown files.

Remember: DON'T PANIC, and always keep your markdown editor handy! 🚀✨ 