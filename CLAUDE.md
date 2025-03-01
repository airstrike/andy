# CLAUDE.md - Development Guide

This document contains important information about the project structure, development workflows, and common commands for the personal blog application built with Rust and Actix-web.

## Common Commands

### Development

```bash
# Run the development server
cargo run

# Run tests
cargo test

# Check for errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Production

```bash
# Build for production
cargo build --release

# Run in production mode
./target/release/andy
```

## Git Workflow

### Branch Strategy

1. **Main Branch**: 
   - The `main` branch always contains production-ready code
   - Never commit directly to `main`
   - All changes must go through pull requests

2. **Feature Branches**:
   - Create a new branch for each feature or bug fix
   - Use descriptive names with prefixes: `feature/`, `bugfix/`, `refactor/`, `docs/`, etc.
   - Example: `feature/add-pagination`, `bugfix/fix-date-format`

3. **Pull Requests**:
   - Create a pull request for each branch
   - Include a clear description of changes
   - Request reviews before merging
   - Squash commits when merging to keep history clean

### Commit Messages

Use semantic commit messages with the following format:

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code changes that neither fix bugs nor add features
- `test`: Adding or improving tests
- `chore`: Changes to build process, tools, etc.

Examples:
```
feat: add syntax highlighting for Ruby
fix: correct date formatting in blog post template
refactor: improve error handling in post parser
docs: update README with deployment instructions
test: add tests for the archive page
```

### Versioning

This project follows Semantic Versioning (SemVer):

- **MAJOR** version for incompatible API changes (x.0.0)
- **MINOR** version for new features in a backward-compatible manner (0.x.0)
- **PATCH** version for backward-compatible bug fixes (0.0.x)

When releasing:
1. Update version in Cargo.toml
2. Create a tag with the version number (e.g., `v1.0.0`)
3. Generate release notes based on commits since the last release

## Code Style Guidelines

1. **Rust Conventions**:
   - Follow the official Rust style guide
   - Use `snake_case` for variable and function names
   - Use `CamelCase` for type names
   - Use `SCREAMING_SNAKE_CASE` for constants
   - Use the `?` operator for error propagation

2. **Error Handling**:
   - Use the `thiserror` crate for defining error types
   - Create enum variants for different error cases
   - Prefer the `?` operator for error propagation
   - Log errors with appropriate context using the `log` crate

3. **Async/Await**:
   - Use async/await for all I/O operations
   - Avoid blocking the async runtime
   - Use `tokio::spawn` for running blocking tasks

4. **Documentation**:
   - Document all public functions with doc comments
   - Explain the purpose, parameters, return values, and possible errors
   - Include examples where appropriate

## Project Structure Details

### Route Handlers

The `routes` module contains all HTTP route handlers:
- `home.rs`: Home page with latest posts
- `about.rs`: About page
- `blog.rs`: Blog post archive and individual post pages

Routes are configured in `routes/mod.rs` and imported in `main.rs`.

### Data Models

The `models` module contains data structures:
- `blog_post.rs`: Represents a blog post with frontmatter parsing and markdown rendering

### Templates

Handlebars templates are in the `templates` directory:
- `layouts/main.hbs`: Base layout template with HTML head, header, and footer
- `index.hbs`: Home page template
- `about.hbs`: About page template
- `blog/archive.hbs`: Blog archive page
- `blog/post.hbs`: Individual blog post page

### Static Assets

CSS and JavaScript files are in the `static` directory and served directly by Actix-web.

## Blog Post Format

Blog posts are Markdown files in the `content` directory with frontmatter:

```markdown
---
title: Your Post Title
date: 2024-03-01T12:00:00Z
description: A short description of your post
slug: your-post-slug
---

Your Markdown content here...
```

Required frontmatter fields:
- `title`: Post title
- `date`: Publication date in RFC3339 format
- `description`: Short post description for meta tags
- `slug`: URL-friendly identifier for the post

## Testing

Integration tests are in the `tests` directory:
- `integration.rs`: Tests for the entire application
- `home.rs`: Tests for the home page
- `about.rs`: Tests for the about page
- `blog.rs`: Tests for blog functionality

## Adding New Features

When adding new features:

1. **Route Handlers**:
   - Add new route handlers to the appropriate module
   - Update `routes/mod.rs` to configure the new routes

2. **Templates**:
   - Create new Handlebars templates for the feature
   - Use the existing layout template for consistency

3. **Models**:
   - Add new data models as needed
   - Follow the existing error handling pattern

4. **Tests**:
   - Write integration tests for the new feature
   - Test both success and error cases

## Deployment

The application is designed to be deployed as a standalone binary. Build for production with `cargo build --release` and run the resulting binary.

For production deployments, consider:
- Setting up a reverse proxy (Nginx, Caddy)
- Configuring TLS/SSL for HTTPS
- Setting up proper logging
- Implementing monitoring