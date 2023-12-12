import  { Universe, Cell } from "game-of-life";
import  { memory } from "game-of-life/game_of_life_bg.wasm";
import bootstrap from 'bootstrap';
import 'bootstrap/dist/css/bootstrap.css';
import './styles.css'


const CELL_SIZE = 6;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
const canvas = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const speedControl = document.getElementById("speed");
const resetButton = document.getElementById("reset");
const randomizeButton = document.getElementById("randomize");


// profiling

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now()
        const delta =  now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if(this.frames.length > 100 ) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }

        let mean = sum / this.frames.length;

        this.fps.textContent = `
Frames per second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
}



//brushes
const brushLabel = document.getElementById("brushLabel")
const brushes = [
    document.getElementById("dot"),
    document.getElementById("box"),
    document.getElementById("beeHive"),
    document.getElementById("loaf"),
    document.getElementById("boat"),
    document.getElementById("tub"),
    document.getElementById("blinker"),
    document.getElementById("toad"),
    document.getElementById("beacon"),
    document.getElementById("pulsar"),
    document.getElementById("pentadeca"),
    document.getElementById("glider"),
    document.getElementById("lwss"),
    document.getElementById("mwss"),
    document.getElementById("glidergen")
]


let shape = "dot"

canvas.height = (CELL_SIZE + 1) * height +1;
canvas.width =  (CELL_SIZE + 1) * height +1;

const ctx = canvas.getContext('2d');

// animation And Interactivity
let animationId = null;
let frameCount = 0;

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

const isPaused = () => {
    return animationId === null;
}

playPauseButton.addEventListener("click", event => {
    if(isPaused()){
        play();
    }else {
        pause();
    }
});

let selectBrush = (event) => {
    brushLabel.textContent = `Selected: ${event.target.textContent}`
    shape = event.target.id.toLowerCase()
    console.log(shape)
}

for (const brush of brushes) {
    brush.addEventListener("click", selectBrush)
}




canvas.addEventListener("click", e => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (e.clientX - boundingRect.left)  * scaleX;
    const canvasTop = (e.clientY - boundingRect.top) * scaleY;
    
    const row = Math.min(Math.floor(canvasTop /(CELL_SIZE + 1)), height -1 );
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width -1);
    if (shape === "dot") {
        universe.toggle_cell(row, col);
    }
    else {
        universe.draw_shape(row, col, shape);
    }

    drawGrid();
    drawCells();
})

speedControl.addEventListener("input",(e)=>{
    const speedLabel = document.getElementById("speedLabel");
    speedLabel.textContent = speedControl.value;
})

resetButton.addEventListener("click", (e)=> {
    universe.reset()
    universe.tick()
    if (isPaused){
        play();
        pause();
    }
});

randomizeButton.addEventListener("click", (e) => {
    universe.randomize();
    universe.tick()
    if (isPaused) {
        play();
        pause();
    }
})






// Render
const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};


const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i<= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1 );
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const  cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row =  0; row < height; row++) {
        for (let col = 0; col < width; col++) {
           const idx = getIndex(row, col);
           
           ctx.fillStyle = bitIsSet(idx, cells)
           ? ALIVE_COLOR
           : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
}



const renderLoop = () => {
    fps.render();
    if ( frameCount >= Math.ceil(parseInt(speedControl.value) / 4) ) {
        universe.tick();
        frameCount=1;
    }else {
        frameCount++;
    }
    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);    
}


drawGrid();
drawCells();
play();


