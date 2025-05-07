# What are .md Files?

## Overview
`.md` files are Markdown files. Markdown is a lightweight markup language with plain text formatting syntax that's easy to write and read. In the UnrealNet project, we use Markdown files extensively for documentation, as seen in the `unrealnet-core/docs/` directory.

## Why UnrealNet Uses Markdown

1. **Developer-Friendly**: Engineers can edit documentation in the same environment they write code
2. **Version Control**: Markdown files work seamlessly with git, allowing us to track documentation changes
3. **Simplicity**: Team members can focus on content rather than formatting
4. **Portability**: Documentation can be viewed on any platform or converted to other formats

## Markdown in UnrealNet

The UnrealNet project uses Markdown files for:

- Project documentation (like the files in `unrealnet-core/docs/`)
- README files that provide an overview of each component
- API documentation
- Implementation guides
- Contribution guidelines

## Common Markdown Syntax Used in UnrealNet Docs

### Headers

Headers are used to create section titles:

```
# UnrealNet: Project Overview & Setup

## Project Architecture

### Core Protocol Infrastructure
```

### Code Blocks

For showing implementation examples or terminal commands:

```rust
pub mod psilambda;
pub mod math;
pub mod routing;
pub mod identity;
pub mod observer;
pub mod uns;
```

```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Lists

For organizing related items:

```
- **χ_ID (Chi ID)**: Identity system based on phase vectors
- **Phase Vectors**: Mathematical representation of routing paths
- **Unreal Naming System (UNS)**: Decentralized naming system

1. Core Protocol Infrastructure (2-core-infrastructure.md)
2. Enhanced Protocols (3-enhanced-protocols.md)
3. Web Integration Layer (4-web-integration.md)
```

### Tables

For structured data like API endpoints or module attributes:

```
| Module | Purpose | Dependencies |
|--------|---------|---------------|
| psilambda | Value transfer | math, identity |
| observer | Verification | routing, math |
```

### Links

For cross-references between documentation files:

```
See the [Enhanced Protocols](3-enhanced-protocols.md) document for more details.
```

## Creating Markdown Files for UnrealNet

### Best Practices

1. **Consistent Structure**: Follow the established documentation pattern
2. **Technical Accuracy**: Ensure documentation accurately reflects the implementation
3. **Diagrams**: Use links to diagrams in the `figures/` directory when helpful
4. **Examples**: Include relevant code examples
5. **Cross-References**: Link related documents for easy navigation

### Tools Used by the UnrealNet Team

- **VSCode**: With Markdown plugins for previewing
- **mdbook**: For building comprehensive documentation websites
- **Mermaid**: For embedded diagrams in documentation
- **GitHub/GitLab**: For rendering Markdown in repository views

## Converting Markdown to Other Formats

The UnrealNet build process can convert Markdown documentation to:

- **HTML**: For web-based documentation
- **PDF**: For formal documentation distribution
- **Man pages**: For command-line tools

## Conclusion

Markdown (.md) files are integral to the UnrealNet project's documentation strategy. They provide a balance of readability, simplicity, and power that makes documentation maintenance straightforward while ensuring the information remains accessible to all team members and users.
