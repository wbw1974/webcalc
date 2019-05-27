import * as wasm from "webcalc"

let calc = wasm.Calc.new()
let ans = calc.calc('a + 2')
console.log(JSON.stringify(ans, null, 2))
ans = calc.calc('a = 40')
console.log(JSON.stringify(ans, null, 2))
