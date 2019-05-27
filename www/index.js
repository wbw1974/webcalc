import * as Calc from "../pkg/webcalc"

let webcalc = Calc.new()
let ret = webcalc.calc("2 + 2")
console.log(JSON.stringify(ret, 0, 2))
