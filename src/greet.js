/**
 * Greeting module for belaf-test-repo
 */

function greet(name = 'World') {
  if (!name || typeof name !== 'string') {
    throw new Error('Name must be a non-empty string')
  }
  return `Hello, ${name}! Welcome to belaf-test-repo.`
}

function farewell(name = 'Friend') {
  if (!name || typeof name !== 'string') {
    throw new Error('Name must be a non-empty string')
  }
  return `Goodbye, ${name}! See you soon.`
}

module.exports = { greet, farewell }
