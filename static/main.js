import {draw, render} from "../pkg/d0.js"

let mouseDown = false


function drawing() {
  console.log('mouse down')
}


document.addEventListener('mousedown', function() {
  mouseDown = true
})

document.addEventListener('mouseup', function() {
  mouseDown = false
})

document.addEventListener('mousemove', function(evt) {
  if (mouseDown === false) {
    return
  }
  console.log(draw(evt.clientX, evt.clientY))
  console.log(render())
})
