/**
 * String utility functions
 */

function reverse(str) {
  if (typeof str !== 'string') return ''
  return str.split('').reverse().join('')
}

function isPalindrome(str) {
  if (typeof str !== 'string') return false
  const cleaned = str.toLowerCase().replace(/[^a-z0-9]/g, '')
  return cleaned === cleaned.split('').reverse().join('')
}

function countWords(str) {
  if (typeof str !== 'string') return 0
  return str.trim().split(/\s+/).filter(Boolean).length
}

module.exports = { reverse, isPalindrome, countWords }
