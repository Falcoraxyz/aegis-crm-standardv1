Generated: 2026-01-12 14:20 UTC

# Threat Model Summary

Attacker can copy CERT, reverse engineer binary, patch verification, debug runtime.

Attacker cannot forge vendor signature without vendor_priv; cannot produce PoP without user_priv.

# Hardening Guidance (for consumer apps)

Multi-point license checks (not just one if statement).

Gate core functionality with license state.

Integrity self-hash in multiple code paths.

Optional decoy computation: invalid license yields subtly wrong output.

Optional clock rollback detection via last_seen_time.

# Do NOT Overclaim

Do NOT claim uncrackable absolute. Position as serverless cryptographic licensing with strong authenticity & anti-keygen.
