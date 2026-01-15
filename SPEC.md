Aegis CRM Standard
Cryptographic Rights Management (Offline-first Licensing)
Version: 1.0 (Genesis)
Status: Final Blueprint
Core Doctrine: Math Trust > Server Trust

0. Scope
0.1 Purpose
Aegis CRM Standard mendefinisikan format dan alur verifikasi lisensi software berbasis kriptografi yang serverless (tanpa database / license server), offline-first, anti-keygen (signature asimetris), portable lintas bahasa/platform, dan privacy-first (tanpa identity tradisional).
0.2 Out of Scope
Sistem pembayaran, distribusi file, DRM always-online, serta klaim anti-crack absolut terhadap patching binary berada di luar cakupan standar ini.
1. Terminology
Vendor: pembuat/penerbit software.
User: pemegang lisensi.
CERT: License Certificate (sertifikat lisensi).
PoP: Proof of Possession, pembuktian kepemilikan private key.
Entitlement: hak akses/fitur.
2. Cryptography Primitives
2.1 Curve: secp256k1.
2.2 Public Key Encoding: Compressed SEC1 format (33 bytes).
2.3 Signature: ECDSA secp256k1. Encoding signature untuk CERT: compact 64 bytes (r||s). DER boleh untuk internal runtime namun CERT wajib compact.
2.4 Hash: SHA-256.
2.5 Randomness: Nonce dan License ID wajib berasal dari CSPRNG.
3. Key Roles
3.1 Vendor Keys
- vendor_priv (32 bytes): private key untuk signing CERT (offline/cold storage).
- vendor_pub (33 bytes): embedded ke aplikasi/library.
Aturan: vendor_priv tidak boleh pernah dikirim dalam distribusi aplikasi. vendor_pub harus tertanam di binary/library.
3.2 User Keys
- user_priv (32 bytes): private key milik user.
- user_pub (33 bytes): public key milik user.
Aturan: user_priv dipegang user (file/key/seed). user_pub tertulis dalam CERT.
4. License Certificate (CERT)
4.1 Encoding Format
CERT wajib menggunakan CBOR Canonical Encoding untuk memastikan byte-stability lintas platform.
4.2 Fields
CERT adalah CBOR map dengan field:
- v: u16 (protocol version = 1)
- pid: tstr (product identifier)
- lid: bstr(32) (license identifier random bytes32)
- iat: u64 (issued-at unix timestamp seconds)
- exp: u64 atau null (expiry unix timestamp atau null)
- upk: bstr(33) (user compressed public key)
- feat: u64 (feature bitmask)
- sig: bstr(64) (vendor signature)
5. Signing Rules
5.1 Payload Definition
Vendor signature dihitung atas semua field kecuali sig.
Payload = {v,pid,lid,iat,exp,upk,feat}
5.2 Signing Procedure
1) payload_bytes = cbor_canonical(payload)
2) digest = sha256(payload_bytes)
3) sig = ecdsa_sign(vendor_priv, digest) -> compact 64 bytes
4) cert = payload + {sig}
6. Verification Procedure (Offline Validation)
Verification terdiri dari 2 tahap wajib: (A) Certificate Authenticity dan (B) Proof of Possession.
6.1 Stage A — Certificate Authenticity Check
Input: vendor_pub + cert
Steps:
1) parse CBOR cert
2) extract sig
3) rebuild payload tanpa sig
4) digest = sha256(cbor_canonical(payload))
5) verify ecdsa signature menggunakan vendor_pub
Jika gagal: CERT_INVALID_SIGNATURE
6.2 Stage B — Proof of Possession (Ownership)
Tujuan: mencegah sharing CERT.
Challenge: App generate nonce = random_bytes(32)
Proof: User membuat pop_sig = ecdsa_sign(user_priv, sha256(nonce))
Verify: App verify pop_sig menggunakan upk dari cert
Jika gagal: POP_INVALID_SIGNATURE
7. Entitlement Rules (feat bitmask)
feat adalah bitmask u64.
Mapping fitur bebas vendor definisikan tetapi sebaiknya dipublikasikan.
Contoh:
- bit0: base access
- bit1: feature pack A
- bit2: feature pack B
- bit10: enterprise mode
8. Expiry Rules
Jika exp = null -> perpetual.
Jika exp != null -> aplikasi wajib enforce now_unix <= exp.
Offline caveat: clock rollback attack.
Mitigation opsional:
- simpan last_seen_time lokal
- jika now < last_seen_time: deny / degrade features
9. Standard Error Codes
Implementasi sebaiknya mengembalikan error standar:
- E_CERT_PARSE
- E_UNSUPPORTED_VER
- E_CERT_SIG
- E_CERT_EXPIRED
- E_POP_SIG
- E_CLOCK_ROLLBACK (opsional)
- E_KEY
- E_CRYPTO
10. Security Guidance (Implementation)
Aegis menjamin keaslian lisensi secara kriptografis, namun implementer harus mengasumsikan client hostile.
Rekomendasi hardening:
- multi-point verification
- avoid single gate boolean
- self-integrity check (hash)
- hide vendor_pub
- optional anti-debug
- decoy computation
11. Compliance
Implementasi dapat disebut Aegis CRM Standard v1.0 compliant jika:
- menggunakan CBOR canonical CERT format
- implement vendor signature verification
- implement PoP challenge-response
- implement expiry check
- expose standardized error codes (minimum subset)
Appendix A — Rust SDK Reference Design (for AI Agent)
Crate: aegis-crm-core
Modules: keys, cert, pop, verify, errors
Key Functions:
keys::vendor_keygen(), keys::user_keygen(), keys::pubkey_from_privkey()
cert::issue_cert(), cert::verify_cert(), cert::encode_cert(), cert::decode_cert()
pop::challenge(), pop::prove(), pop::verify()
verify::verify_license()
Dependencies (suggested): k256, sha2, rand_core, serde, serde_cbor, thiserror
Appendix B — CLI Spec (optional)
Crate: aegis-crm-cli
Commands:
- aegis vendor keygen
- aegis user keygen
- aegis issue --vendor-priv vendor.key --user-pub user.pub --pid <id> --feat <mask> --exp <unix|null>
- aegis verify --vendor-pub vendor.pub --cert license.cert
