// belaf-test-repo
// Test project for belaf release management

const { greet, farewell } = require('./src/greet')
const { capitalize, slugify } = require('./src/utils')
const { add, subtract, multiply, divide } = require('./src/math')

module.exports = {
  name: 'belaf-test-repo',
  version: require('./package.json').version,
  greet,
  farewell,
  capitalize,
  slugify,
  add,
  subtract,
  multiply,
  divide
}
