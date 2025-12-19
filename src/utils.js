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

/**
 * Debounce function - delays execution until after wait milliseconds have elapsed
 * @param {Function} fn - The function to debounce
 * @param {number} wait - The number of milliseconds to delay
 * @returns {Function} - The debounced function
 */
function debounce(fn, wait = 300) {
  let timeoutId = null
  return function (...args) {
    clearTimeout(timeoutId)
    timeoutId = setTimeout(() => fn.apply(this, args), wait)
  }
}

/**
 * Throttle function - limits execution to at most once per wait milliseconds
 * @param {Function} fn - The function to throttle
 * @param {number} wait - The minimum time between executions
 * @returns {Function} - The throttled function
 */
function throttle(fn, wait = 300) {
  let lastCall = 0
  return function (...args) {
    const now = Date.now()
    if (now - lastCall >= wait) {
      lastCall = now
      return fn.apply(this, args)
    }
  }
}

module.exports = { capitalize, slugify, truncate, debounce, throttle }
