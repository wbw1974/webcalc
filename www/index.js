import { Calc } from "webcalc"

const calc = Calc.new()
const calcButton = document.getElementById("calcButton");
calcButton.addEventListener("click", event => {
    work()
})

const work = () => {
    console.log("Got to work")
    let input = document.getElementById("input").value
    let ans = calc.calc(input)
    if (ans.state === "success") {
        let output = document.getElementById("output").value=ans.value
    }
}
