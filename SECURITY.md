# Security Policy

## Supported Versions

We actively support security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.3.x   | :white_check_mark: |
| 0.2.x   | :x:                |
| 0.1.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in MCP Rust SDK, please report it responsibly:

### How to Report

1. **Do NOT create a public GitHub issue** for security vulnerabilities
2. Send an email to: [your-security-email@example.com]
3. Include as much detail as possible:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days with preliminary assessment
- **Resolution Timeline**: Varies by severity, typically 30-90 days

### Security Best Practices

When using MCP Rust SDK:

1. **Keep Dependencies Updated**: Regularly run `cargo update` and `cargo audit`
2. **Validate Input**: Always validate data from external sources
3. **Use TLS**: Enable TLS for HTTP and WebSocket transports in production
4. **Limit Permissions**: Run with minimal required permissions
5. **Monitor Dependencies**: Use `cargo-deny` to check for security advisories

### Security Features

MCP Rust SDK includes several security features:

- Input validation for all protocol messages
- Safe deserialization with serde
- Memory-safe Rust code
- Optional TLS support for network transports
- Resource access controls

### Dependency Security

We use automated tools to monitor dependencies:

- `cargo-audit` for security advisories
- `cargo-deny` for license and dependency checking
- Dependabot for automated updates

### Security Testing

Our security testing includes:

- Static analysis with Clippy
- Dependency vulnerability scanning
- Fuzzing of protocol parsing (planned)
- Memory safety verification

## Security Contacts

- Security Team: [security@example.com]
- Project Maintainer: [maintainer@example.com]

Thank you for helping keep MCP Rust SDK secure!
