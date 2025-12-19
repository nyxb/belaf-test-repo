// belaf-test-repo
// Test project for belaf release management

const { greet, farewell } = require('./src/greet')
const { capitalize, slugify } = require('./src/utils')

module.exports = {
  name: 'belaf-test-repo',
  version: require('./package.json').version,
  greet,
  farewell,
  capitalize,
  slugify
}
