// user-service.js — User service layer
// Part of the demo-repo example for AI Dev Env

const db = require('./db');

/**
 * Get a user by ID.
 */
async function getUserById(id) {
  const result = await db.query(
    'SELECT id, username, email, created_at FROM users WHERE id = $1',
    [id]
  );
  return result.rows[0] || null;
}

/**
 * Update a user's email address.
 */
async function updateUserEmail(userId, newEmail) {
  if (!newEmail || !newEmail.includes('@')) {
    throw new Error('Invalid email address');
  }
  await db.query(
    'UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2',
    [newEmail, userId]
  );
}

module.exports = { getUserById, updateUserEmail };
