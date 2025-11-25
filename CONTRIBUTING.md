# Contributing to tulna-rs

Thank you for your interest in contributing to tulna-rs! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)

## Code of Conduct

We are committed to providing a welcoming environment for all contributors. Please be respectful and considerate in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/tulna-rs.git
   cd tulna-rs
   ```
3. **Add the upstream repository** as a remote:
   ```bash
   git remote add upstream https://github.com/SolidLabResearch/tulna-rs.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later (2021 edition)
- Cargo (comes with Rust)

### Building the Project

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for a specific module
cargo test graph_isomorphism
```

### Running Examples

```bash
# Run graph isomorphism examples
cargo run --example graph_isomorphism

# Run query isomorphism examples
cargo run --example query_isomorphism
```

## Making Changes

### Branching Strategy

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. Keep your branch up to date with upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### Commit Messages

Write clear, descriptive commit messages:

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests when relevant

Example:
```
Add support for SPARQL 1.1 UNION operator

- Implement UNION pattern parsing
- Add test cases for UNION queries
- Update documentation

Fixes #123
```

## Testing

All contributions should include appropriate tests:

### Unit Tests

- Place unit tests in a `tests` module at the bottom of the source file
- Use descriptive test names that explain what is being tested
- Test both success and failure cases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works_correctly() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = your_function(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_feature_handles_edge_case() {
        // Test edge cases
    }
}
```

### Integration Tests

- Place integration tests in the `tests/` directory
- Create separate files for different feature areas
- Use realistic test scenarios

### Test Coverage

Aim for:
- 100% coverage of public APIs
- High coverage of critical paths
- Edge case coverage

Run tests before submitting:
```bash
cargo test
```

## Code Style

### Formatting

We use `rustfmt` for consistent code formatting:

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check
```

### Linting

We use `clippy` for catching common mistakes and improving code quality:

```bash
# Run clippy
cargo clippy

# Run clippy with warnings as errors (required for CI)
cargo clippy -- -D warnings
```

**All clippy warnings must be resolved before submitting a PR.**

### Documentation

- Document all public APIs with `///` doc comments
- Include examples in doc comments when helpful
- Explain complex algorithms with module-level documentation
- Update README.md if adding new features

```rust
/// Check if two RDF graphs are isomorphic.
///
/// This method uses a hash-based grounding algorithm to efficiently
/// determine structural equivalence.
///
/// # Arguments
///
/// * `graph1` - First RDF graph as a slice of triples
/// * `graph2` - Second RDF graph as a slice of triples
///
/// # Returns
///
/// * `Ok(true)` - Graphs are isomorphic
/// * `Ok(false)` - Graphs are not isomorphic
///
/// # Examples
///
/// ```
/// use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};
///
/// let graph1 = vec![/* ... */];
/// let graph2 = vec![/* ... */];
/// assert!(GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
/// ```
pub fn are_isomorphic(graph1: &[Triple], graph2: &[Triple]) -> Result<bool, Error> {
    // Implementation
}
```

### Rust Best Practices

- Use idiomatic Rust patterns
- Prefer `Result` over `panic!` for error handling
- Use type inference where it improves readability
- Avoid unnecessary allocations
- Use `&str` for string slices, `String` for owned strings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Submitting Changes

### Pull Request Process

1. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

2. **Update documentation**:
   - Add/update doc comments
   - Update README.md if needed
   - Update CHANGELOG.md under `[Unreleased]` section

3. **Create a pull request**:
   - Provide a clear title and description
   - Reference related issues
   - Explain what changes were made and why
   - Include screenshots for UI changes (if applicable)

4. **Code review**:
   - Address reviewer feedback promptly
   - Keep discussions focused and professional
   - Be open to suggestions and alternatives

5. **Merge**:
   - Squash commits if requested
   - Ensure CI passes
   - Maintainers will merge when approved

### Pull Request Template

```markdown
## Description
Brief description of what this PR does.

## Related Issue
Fixes #(issue number)

## Changes Made
- Change 1
- Change 2

## Testing
- [ ] All tests pass
- [ ] New tests added
- [ ] Clippy warnings resolved
- [ ] Code formatted with rustfmt

## Documentation
- [ ] Doc comments added/updated
- [ ] README updated (if needed)
- [ ] CHANGELOG updated
```

## Reporting Issues

### Bug Reports

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to reproduce**: Minimal code example
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**:
   - Rust version (`rustc --version`)
   - OS and version
   - tulna-rs version

Example:
```markdown
**Description**
Graph isomorphism fails for graphs with symmetric structure.

**Steps to Reproduce**
```rust
let graph1 = vec![/* ... */];
let graph2 = vec![/* ... */];
let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2);
```

**Expected**: Should return `Ok(true)`
**Actual**: Returns `Ok(false)`

**Environment**
- Rust: 1.70.0
- OS: macOS 14.0
- tulna-rs: 0.1.0
```

## Feature Requests

We welcome feature requests! Please:

1. Check if the feature has already been requested
2. Provide a clear use case
3. Explain why it would be valuable
4. Consider implementation complexity
5. Be open to discussion about alternatives

## Areas for Contribution

### Good First Issues

Look for issues labeled `good first issue` for beginner-friendly tasks.

### Areas Needing Help

- Performance optimizations
- Additional query language support
- Improved error messages
- Documentation improvements
- Example additions
- Test coverage improvements

## Questions?

- Open an issue with the `question` label
- Check existing documentation and issues first

## License

By contributing to tulna-rs, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to tulna-rs!