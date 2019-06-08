import { Calc } from "webcalc"

const calc = Calc.new()
const calcButton = document.getElementById("calcButton");
calcButton.addEventListener("click", event => {
    work()
})

const work = () => {
    let input = document.getElementById("input").value
    var history = document.getElementById("inputHistory")
    if(history.selectionStart == history.selectionEnd) {
        history.scrollTop = history.scrollHeight;
    }
    history.value += input + "\n"
    
    let ans = calc.calc(input)
    console.log("calculation: " + JSON.stringify(ans, null, 2))
    if (ans.state === "success") {
        document.getElementById("output").value=ans.value
    }
}
