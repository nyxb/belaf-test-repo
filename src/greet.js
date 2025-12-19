/**
 * Greeting functions
 */

function greet(name = 'World') {
  if (typeof name !== 'string') {
    throw new TypeError('Name must be a string')
  }
  return `Hello, ${name}!`
}

function farewell(name = 'World') {
  if (typeof name !== 'string') {
    throw new TypeError('Name must be a string')
  }
  return `Goodbye, ${name}!`
}

function welcome(name = 'Guest', location = 'here') {
  if (typeof name !== 'string' || typeof location !== 'string') {
    throw new TypeError('Arguments must be strings')
  }
  return `Welcome to ${location}, ${name}!`
}

module.exports = { greet, farewell, welcome }
