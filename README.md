# Exo Directory

**The 100 % open-source, fully-compatible Active Directory replacement.**

No licenses. No lock-in. Runs anywhere. Domain-joins Windows 10/11/Server natively. Full Group Policy engine. Modern web UI. Built for 2025 and beyond.

🚀 **Goal**: Be the first directory that lets any organization — enterprise, school, government, startup — completely remove Microsoft Active Directory without pain or compromise.

## Why Exo exists
- Samba is amazing but GPO support is still ~95 %
- FreeIPA is great for Linux but Windows clients are second-class
- Entra ID is cloud-only and still missing huge chunks
- Univention UCS is the closest, but Europe-only and still Samba-based

Exo starts where they stop: a ground-up, LLM-accelerated, Rust-core implementation that closes the last 5 % from day one.

## Status
**November 21 2025 — Day Zero**  
We just shipped the first real code.

## Quick start (coming in the next hours)
```bash
docker run -p 389:389 -p 88:88 ghcr.io/exodirectory/exo:latest
