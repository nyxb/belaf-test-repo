/**
 * Object utility functions
 */

function deepClone(obj) {
  if (obj === null || typeof obj !== 'object') return obj
  return JSON.parse(JSON.stringify(obj))
}

function pick(obj, keys) {
  if (!obj || typeof obj !== 'object') return {}
  return keys.reduce((acc, key) => {
    if (key in obj) acc[key] = obj[key]
    return acc
  }, {})
}

function omit(obj, keys) {
  if (!obj || typeof obj !== 'object') return {}
  const keysSet = new Set(keys)
  return Object.fromEntries(
    Object.entries(obj).filter(([key]) => !keysSet.has(key))
  )
}

module.exports = { deepClone, pick, omit }
