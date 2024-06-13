const fontSizeMap = require('../font-sizes.json');

const text = 'Nå|Venter kun ett rentekutt i år';
let textLength = 0;

for (const char of text.split('')) {
    textLength += fontSizeMap[char]
}

console.log(`Text length: ${textLength}`)

console.log(`Font size: ${textLength}`)