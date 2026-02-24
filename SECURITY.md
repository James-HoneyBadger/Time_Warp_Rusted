# Security Policy

## Security Model

Time Warp Rusted is a native desktop application.  **It does not expose any network services**, does not communicate over the network, and does not execute code outside its own process.

The language executors (BASIC, PILOT, Logo, C, Pascal, Prolog, Forth) are **interpreters written in safe Rust** — they operate on their own `ExecContext` state and cannot access the filesystem, spawn processes, or make system calls.  There is no `unsafe` code in any executor.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.x (Rust edition) | Yes |
| 7.x (Rust edition - legacy) | Maintenance only |
| 6.x and earlier (Python edition) | End-of-life |

## Reporting a Vulnerability

If you discover a security issue please **do not open a public GitHub issue**.

1. Email a description to the project maintainer (see the GitHub profile).
2. Include: affected version, reproduction steps, and potential impact.
3. You will receive an acknowledgement within 48 hours.
4. A fix will be released and you will be credited (unless you prefer anonymity).

## Scope

Applicable vulnerability classes for a desktop interpreter:

- Memory safety bugs that could allow escape from the interpreter sandbox
- Malformed input causing panics or infinite loops that cannot be stopped
- Editor widget vulnerabilities from the egui/eframe dependency chain

Out of scope:

- Social engineering attacks
- Issues in programs *run by* the interpreter (they are sandboxed by design)
- Denial-of-service via large inputs (the executor has a `max_iterations` limit)
