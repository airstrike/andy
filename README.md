# Personal Blog

A simple, modern blog built with Rust and Actix-web. This project uses Handlebars for templating, vanilla JavaScript for frontend interactions, and Markdown for content authoring.

## Features

- Responsive design that works on all devices
- Markdown-based blog posts with frontmatter
- Clean and minimalist design
- Fast load times thanks to Rust and Actix-web
- Simple architecture for easy maintenance

## Project Structure

```
/
├── content/             # Markdown blog posts
├── static/              # Static assets (CSS, JavaScript, images)
│   ├── css/
│   ├── js/
│   └── img/
├── templates/           # Handlebars templates
│   ├── layouts/
│   └── blog/
└── src/                 # Rust source code
    ├── models/          # Data models
    └── routes/          # Route handlers
```

## Running Locally

1. Clone the repository
2. Install Rust if you haven't already: https://www.rust-lang.org/tools/install
3. Run the development server:

```bash
cargo run
```

The server will start at http://localhost:8080

## Creating Blog Posts

Blog posts are written in Markdown with frontmatter. Create a new .md file in the `content` directory with the following structure:

```markdown
---
title: Your Post Title
date: 2024-03-01T12:00:00Z
description: A short description of your post
slug: your-post-slug
---

Your Markdown content here...
```

## Build for Production

To build the project for production:

```bash
cargo build --release
```

The binary will be available in `target/release/andy`

## License

MIT