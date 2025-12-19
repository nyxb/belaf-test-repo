/**
 * Array utility functions
 */

function unique(arr) {
  return [...new Set(arr)]
}

function flatten(arr) {
  return arr.flat(Infinity)
}

function chunk(arr, size = 1) {
  const result = []
  for (let i = 0; i < arr.length; i += size) {
    result.push(arr.slice(i, i + size))
  }
  return result
}

/**
 * Shuffle array using Fisher-Yates algorithm
 * @param {Array} arr - Array to shuffle
 * @returns {Array} - Shuffled array
 */
function shuffle(arr) {
  const result = [...arr]
  for (let i = result.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    ;[result[i], result[j]] = [result[j], result[i]]
  }
  return result
}

/**
 * Get intersection of two arrays
 * @param {Array} arr1 - First array
 * @param {Array} arr2 - Second array
 * @returns {Array} - Elements present in both arrays
 */
function intersection(arr1, arr2) {
  const set2 = new Set(arr2)
  return arr1.filter(item => set2.has(item))
}

/**
 * Get difference of two arrays (arr1 - arr2)
 * @param {Array} arr1 - First array
 * @param {Array} arr2 - Second array
 * @returns {Array} - Elements in arr1 but not in arr2
 */
function difference(arr1, arr2) {
  const set2 = new Set(arr2)
  return arr1.filter(item => !set2.has(item))
}

module.exports = { unique, flatten, chunk, shuffle, intersection, difference }
