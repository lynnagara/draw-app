import {init} from "../pkg/d0.js"

const {clientWidth, clientHeight} = document.body;

const width = Math.min(Math.max(clientWidth, 600), 3000)
const height = Math.min(Math.max(clientHeight, 400), 2000)

init(width, height)

