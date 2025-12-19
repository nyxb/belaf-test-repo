# belaf-test-repo

Test repository for testing the belaf CLI and GitHub App.

## Installation

```bash
npm install belaf-test-repo
```

## Usage

```javascript
const { greet, farewell } = require('belaf-test-repo/src/greet')
const { capitalize, slugify } = require('belaf-test-repo/src/utils')

console.log(greet('Developer'))  // Hello, Developer! Welcome to belaf-test-repo.
console.log(farewell('Developer'))  // Goodbye, Developer! See you soon.
console.log(capitalize('hello'))  // Hello
console.log(slugify('Hello World'))  // hello-world
```

## License

MIT
