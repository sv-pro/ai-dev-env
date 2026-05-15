# Sample Branch Delta Review

> This is the expected output when the `branch-delta-review` skill runs against
> `examples/demo-repo/branch.diff`. The same output is produced regardless of
> which agent runtime (Claude Code, Codex, Gemini CLI) executes the review.

---

## Summary

This branch introduces an authentication module (`auth.js`) and a user controller
(`user-controller.js`). The authentication module adds login, session token
generation, and session validation functionality. However, the implementation
contains a critical SQL injection vulnerability in `authenticateUser`, a security
weakness in `generateSessionToken` (using `Math.random()` instead of a
cryptographically secure source), and a clear architecture violation in
`user-controller.js` which queries the database directly instead of going through
the service layer. These issues must be resolved before merging.

---

## Bug Findings

- [CRITICAL] `src/auth.js:10` — SQL injection vulnerability. Username and password
  are interpolated directly into the SQL query string without parameterization.
  An attacker can log in as any user by providing `' OR '1'='1` as the username.
  Suggestion: Use parameterized queries:
  ```js
  const result = await db.query(
    'SELECT * FROM users WHERE username = $1 AND password = $2',
    [username, password]
  );
  ```

- [CRITICAL] `src/user-controller.js:11` — SQL injection vulnerability in
  controller. The login controller also interpolates `username` directly into
  a SQL string.
  Suggestion: Use parameterized queries and delegate to `user-service.js`.

- [HIGH] `src/auth.js:19` — `Math.random()` is not cryptographically secure.
  Session tokens generated with `Math.random()` are predictable and could be
  guessed by an attacker.
  Suggestion: Use `crypto.randomBytes(32).toString('hex')` from Node's built-in
  `crypto` module.

---

## Architecture Concerns

- [HIGH] `src/user-controller.js:5,11` — Architecture layer violation.
  The controller imports `db` directly and executes raw SQL queries. Controllers
  must not access the database directly; they must go through the service layer.
  Reference: `shared/architecture-rules.md` — Layered Architecture, Layer Rules
  > "Adapters may reference Skills and Shared. They must not contain workflow logic."
  In application terms: controllers must call services; services call repositories/db.
  Suggestion: Replace direct `db.query` in the controller with a call to
  `user-service.js#getUserByCredentials` (a new function to be added).

---

## Risky Changes

- [CRITICAL] `src/auth.js` — New authentication code with SQL injection.
  Risk: This code, if deployed, would allow any user to bypass authentication
  entirely using a trivial SQL injection payload.
  Mitigation: Fix SQL injection before merging. Do not deploy to any environment.

- [HIGH] `src/auth.js:19` — Weak session token entropy.
  Risk: Session tokens could be predicted by an attacker, allowing session
  hijacking without credential theft.
  Mitigation: Replace `Math.random()` with `crypto.randomBytes`.

- [MEDIUM] No tests added for `auth.js` or `user-controller.js`.
  Risk: Authentication code without test coverage is high-risk. Regressions
  could go undetected.
  Mitigation: Add unit tests for `authenticateUser`, `generateSessionToken`,
  and `validateSession` before merging.

---

## Recommendation

**REQUEST_CHANGES**

Rationale: Two CRITICAL SQL injection vulnerabilities and one HIGH architecture
violation must be resolved before this branch can be merged.
