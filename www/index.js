import * as wasm from "webcalc"

let calc = wasm.Calc.new()
let ans = calc.calc('a * a')
let range = Array.from(new Array(21), (x, i) => i + -10)
console.log(JSON.stringify(range, null, 2))    
for (let item of range) {
    ans = calc.calc('a = ' + item)
    console.log(JSON.stringify(ans, null, 2))    
}
