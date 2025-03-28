# RustyGit

A high-performance Git operations library written in Rust with Python bindings.

## Vision & Goals

### Rust Performance & Parallelism
- Leverage Rust's speed and memory safety for Git operations
- Provide true concurrency and parallel Git operations using Rayon and Tokio
- Dramatically improve performance for CPU-bound and I/O-bound Git tasks

### Pythonic Interface
- Offer simple, intuitive Python APIs similar to GitPython, but faster and more robust
- Use PyO3 to seamlessly bridge Python and Rust
- Maintain Pythonic conventions while benefiting from Rust's performance

### Compatibility & Extensibility
- Aim for compatibility with popular GitPython use cases
- Provide easy-to-use classes for Repositories, Commits, Branches, contributors, etc.
- Build a foundation for Git operations that can be extended for various workflows

## Tech Stack

### Rust Crates
- **git2**: Excellent Git bindings for libgit2
- **rayon**: For easy, reliable parallelism with work-stealing
- **tokio**: Asynchronous runtime for non-blocking I/O operations
- **thiserror**: Expressive error handling

### Python Bindings
- **PyO3**: Rust bindings for the Python interpreter

### Packaging and Distribution
- **maturin**: Build and publish crates with Python bindings

## Quick Example

```python
import rustygit

# Open a repository
repo = rustygit.Repository("/path/to/repo")

# Check if it's a bare repository
is_bare = repo.is_bare()
```

For complete documentation and examples, please refer to our [Sphinx documentation](https://rustygit.readthedocs.io/).

## Installation

```bash
pip install rustygit
```

## Development Status

RustyGit is currently in early development. We're actively working on implementing core functionality and optimizing performance.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.