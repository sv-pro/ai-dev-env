// auth.js — Authentication module
// Part of the demo-repo example for AI Dev Env

const db = require('./db');

/**
 * Authenticate a user by username and password.
 * Returns the user object if authentication succeeds, null otherwise.
 */
async function authenticateUser(username, password) {
  // BUG: SQL injection vulnerability — username is not sanitized
  // This is intentional for the demo review to catch
  const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
  const result = await db.query(query);
  return result.rows[0] || null;
}

/**
 * Generate a session token for an authenticated user.
 */
function generateSessionToken(userId) {
  // Using Math.random() for token generation is not cryptographically secure
  // This is intentional for the demo review to catch
  return `session_${userId}_${Math.random().toString(36).substr(2)}`;
}

/**
 * Validate that a session token is still active.
 */
async function validateSession(token) {
  const result = await db.query(
    'SELECT * FROM sessions WHERE token = $1 AND expires_at > NOW()',
    [token]
  );
  return result.rows.length > 0;
}

module.exports = { authenticateUser, generateSessionToken, validateSession };
