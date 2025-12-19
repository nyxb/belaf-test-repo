/**
 * Utility functions
 */

function capitalize(str) {
  if (typeof str !== 'string' || str.length === 0) return ''
  return str.charAt(0).toUpperCase() + str.slice(1)
}

function slugify(str) {
  if (typeof str !== 'string') return ''
  return str.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')
}

function truncate(str, length = 50) {
  if (typeof str !== 'string') return ''
  return str.length > length ? str.slice(0, length) + '...' : str
}

module.exports = { capitalize, slugify, truncate }
