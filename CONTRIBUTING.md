# Contributing to Global Clipboard Manager

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/global_clipboard.git
   cd global_clipboard
   ```
3. **Install dependencies**:
   ```bash
   bun install
   ```
4. **Create a branch** for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 📝 Code Style

### Rust (Backend)
- Follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality

### TypeScript/Svelte (Frontend)
- Use TypeScript for all new code
- Follow the existing Svelte 5 Runes patterns
- Run `bun run check` to ensure types are correct
- Use Prettier for formatting: `bunx prettier --write src`

### Commit Messages
Use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Test additions or changes
- `chore:` - Build process or auxiliary tool changes

Examples:
```
feat: add syntax highlighting for Python code
fix: resolve hotkey conflict on Windows
docs: update installation instructions
```

## 🧪 Testing

### Run Tests
```bash
# Rust tests
cargo test

# Type checking
bun run check
```

### Manual Testing
1. Test on your target platform (macOS, Windows, or Linux)
2. Verify the app starts correctly
3. Test the feature you added/modified
4. Check for console errors
5. Test edge cases

## 📦 Pull Request Process

1. **Update documentation** if needed (README, inline comments)
2. **Add tests** for new features
3. **Run all checks**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   bun run check
   ```
4. **Create a Pull Request** with:
   - Clear title describing the change
   - Description of what changed and why
   - Screenshots/videos for UI changes
   - Link to related issues

### PR Checklist
- [ ] Code follows the project style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] No merge conflicts
- [ ] PR description is clear and complete

## 🐛 Reporting Bugs

### Before Submitting
- Check if the bug already exists in [Issues](https://github.com/yourusername/global_clipboard/issues)
- Try to reproduce on the latest version
- Collect relevant information (OS, version, logs)

### Bug Report Template
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
 - OS: [e.g. macOS 14.0]
 - App Version: [e.g. 1.0.0]
 - Browser: [if relevant]

**Additional context**
Any other context about the problem.
```

## 💡 Suggesting Features

### Feature Request Template
```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
Other solutions or features you've considered.

**Additional context**
Mockups, examples, or other context.
```

## 🏗️ Architecture Guidelines

### Frontend (Svelte 5)
- Use Runes (`$state`, `$derived`, `$effect`) for reactivity
- Keep components small and focused
- Use the repository pattern for data access
- Follow Clean Architecture principles

### Backend (Rust)
- Keep commands simple and focused
- Use the repository pattern for database access
- Handle errors properly with `Result<T, String>`
- Add logging for debugging

### Database
- Use migrations for schema changes
- Keep queries in the repository layer
- Use pagination for large datasets

## 🔐 Security

- **Never commit secrets** (API keys, passwords, etc.)
- Use `.gitignore` properly
- Report security issues privately to the maintainers
- Follow Tauri's security best practices

## 📚 Resources

- [Tauri Documentation](https://tauri.app)
- [Svelte 5 Documentation](https://svelte.dev)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)

## 🤝 Code of Conduct

### Our Pledge
We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards
- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior
- Harassment or discrimination
- Trolling or insulting comments
- Personal or political attacks
- Publishing others' private information

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Questions?** Feel free to ask in [Discussions](https://github.com/yourusername/global_clipboard/discussions) or open an issue!

Thank you for contributing! 🙏
