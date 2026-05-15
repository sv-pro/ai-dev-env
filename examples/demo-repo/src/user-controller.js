// user-controller.js — HTTP controller for user endpoints
// Part of the demo-repo example for AI Dev Env
//
// NOTE: This file contains intentional architecture violations for demo purposes.
// It demonstrates what the branch-delta-review skill catches.

const db = require('./db');
const { generateSessionToken } = require('./auth');

// ARCHITECTURE VIOLATION: controller is querying DB directly instead of
// going through the user-service layer (see shared/architecture-rules.md)
async function loginController(req, res) {
  const { username, password } = req.body;

  // BUG: SQL injection — username is not sanitized
  const query = `SELECT * FROM users WHERE username = '${username}'`;
  const result = await db.query(query);
  const user = result.rows[0];

  if (!user || user.password !== password) {
    return res.status(401).json({ error: 'Invalid credentials' });
  }

  const token = generateSessionToken(user.id);
  res.json({ token });
}

module.exports = { loginController };
