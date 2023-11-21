import { Universe, Cell } from "wasm-game-of-life";

// We require direct access to WASM-memory..
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

// # CONSTANTS
const CELL_SIZE = 6; // px
const GRID_COLOR = "#F5F5F5";
const DEAD_COLOR = "#D2FBF2";
const ALIVE_COLOR = "#4A148C";

// # FUNCTIONS
const em_parse = (value) => {
    const em = parseFloat(
        getComputedStyle(
            document.querySelector('body')
        )['font-size']
    );

    return Math.floor(value / em);
}
const delay = ms => new Promise(res => setTimeout(res, ms));

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

// # START

const universe = Universe.new(
    em_parse(window.innerWidth) * 2,
    em_parse(window.innerHeight) * 2
);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

// animation state
let animationId = null;
const isPaused = () => {
    return animationId === null;
};

// Event Listeners
const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

// to toggle cell state on click
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    const place_glider = document.getElementById("glider").checked;
    const place_church = document.getElementById("church").checked;
    const place_heart = document.getElementById("heart").checked;
        
    if (place_heart === true) {
        universe.place_heart(row, col);
    } else if (place_glider === true) {
        universe.place_glider(row, col);
    } else if (place_church === true) {
        universe.place_church(row, col);
    } else {
        universe.toggle_cell(row, col);
    }


    drawGrid();
    drawCells();
});



// Actual Game Loop..
const renderLoop = () => {
    // debugger; // opens debugger in browser.
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};
// Initialize
play();

