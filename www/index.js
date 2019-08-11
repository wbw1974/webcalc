import { Calc } from "webcalc"

if (process.env.NODE_ENV !== 'production') {
    console.log('Looks like we are in development mode!')
}
    
const calc = new Calc()
const calcButton = document.getElementById("calcButton")
calcButton.addEventListener("click", event => {
    work()
})

const work = () => {
    calc.calc()
}
