var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
function copyIndex(idx) {
    return { x: idx.x, y: idx.y };
}
function indexEqual(i1, i2) {
    return i1.x === i2.x && i1.y === i2.y;
}
const DirectionKeys = ["right", "left", "down", "up"];
const Direction = {
    right: 0,
    left: 1,
    down: 2,
    up: 3,
};
function isDirKey(str) {
    return DirectionKeys.slice().indexOf(str) !== -1;
}
function areOpposite(dir1, dir2) {
    if (dir1 === undefined || dir2 === undefined)
        return false;
    return ((dir1 === Direction.right && dir2 === Direction.left) ||
        (dir1 === Direction.left && dir2 === Direction.right) ||
        (dir1 === Direction.down && dir2 === Direction.up) ||
        (dir1 === Direction.up && dir2 === Direction.down));
}
const PAGE = {
    frame: 0,
    rows: undefined,
    cols: undefined,
    grid_matrix: [],
    snake_elems: [],
    bits: [],
    grow_snake: false,
    snake: [],
    is_dead: false,
    getSnakeDir: function () {
        let head = PAGE.snake[0];
        let tail = PAGE.snake[1];
        if (tail.x < head.x)
            return Direction.right;
        if (tail.x > head.x)
            return Direction.left;
        if (tail.y < head.y)
            return Direction.down;
        if (tail.y > head.y)
            return Direction.up;
        throw "Error: Invalid snake positions.";
    },
    events: {
        key_down: {
            right: false,
            left: false,
            down: false,
            up: false,
        }
    }
};
function indexInGrid(idx) {
    return (idx.x >= 0 &&
        idx.y >= 0 &&
        idx.x < PAGE.cols &&
        idx.y < PAGE.rows);
}
function makeGrid(rows, cols) {
    const grid = document.getElementById("grid");
    if (grid === null)
        throw "Error: Grid not found.";
    PAGE.cols = cols;
    PAGE.rows = rows;
    for (let n_row = 0; n_row < rows; n_row++) {
        const row_elem = document.createElement("div");
        row_elem.className = "row";
        const row_array = [];
        for (let n_col = 0; n_col < cols; n_col++) {
            const cell = document.createElement("div");
            cell.className = ((n_col + n_row) % 2 === 0)
                ? "cell_even"
                : "cell_odd";
            row_array.push(cell);
            row_elem.appendChild(cell);
        }
        PAGE.grid_matrix.push(row_array);
        grid.appendChild(row_elem);
    }
}
function getGridTile(pos) {
    if (pos)
        return getElementDims(PAGE.grid_matrix[pos.y][pos.x]);
}
function clearSnake() {
    if (PAGE.snake_elems !== undefined) {
        for (const snake_elem of PAGE.snake_elems) {
            snake_elem.remove();
        }
    }
    PAGE.snake_elems = [];
}
function drawImage(image) {
    const img = document.createElement("img");
    img.style.position = "absolute";
    img.src = image.src;
    img.width = image.dims.size.width;
    img.height = image.dims.size.height;
    img.style["margin-left"] = `${image.dims.pos.x}px`;
    img.style["margin-top"] = `${image.dims.pos.y}px`;
    img.style.transform = `rotate(${image.rotation}deg)`;
    return img;
}
function getCellImage(n_cell, snake) {
    const root = "media/";
    if (n_cell === 0) {
        let src = `${root}head.png`;
        let curr = snake[n_cell];
        let next = snake[n_cell + 1];
        if (next.x === curr.x) {
            if (next.y > curr.y)
                return { src: src, rotation: 270 };
            if (next.y < curr.y)
                return { src: src, rotation: 90 };
            throw "Error: Invalid snake positions.";
        }
        if (next.y === curr.y) {
            if (next.x > curr.x)
                return { src: src, rotation: 180 };
            if (next.x < curr.x)
                return { src: src, rotation: 0 };
            throw "Error: Invalid snake positions.";
        }
        throw "Error: Invalid snake positions.";
    }
    else if (n_cell === snake.length - 1) {
        let src = `${root}tail.png`;
        let curr = snake[n_cell];
        let prev = snake[n_cell - 1];
        if (prev.x === curr.x) {
            if (prev.y > curr.y)
                return { src: src, rotation: 180 };
            if (prev.y < curr.y)
                return { src: src, rotation: 0 };
            throw "Error: Invalid snake positions.";
        }
        if (prev.y === curr.y) {
            if (prev.x > curr.x)
                return { src: src, rotation: 90 };
            if (prev.x < curr.x)
                return { src: src, rotation: 270 };
            throw "Error: Invalid snake positions.";
        }
        throw "Error: Invalid snake positions.";
    }
    else {
        let curr = snake[n_cell];
        let next = snake[n_cell + 1];
        let prev = snake[n_cell - 1];
        if (prev.x === next.x)
            return { src: `${root}turn_180.png`, rotation: 90 };
        if (prev.y === next.y)
            return { src: `${root}turn_180.png`, rotation: 0 };
        let src = `${root}turn_90.png`;
        for (const [a, b] of [[prev, next], [next, prev]]) {
            if (a.x < curr.x) {
                if (b.y < curr.y)
                    return { src: src, rotation: 180 };
                if (b.y > curr.y)
                    return { src: src, rotation: 90 };
            }
            if (a.x > curr.x) {
                if (b.y < curr.y)
                    return { src: src, rotation: 270 };
                if (b.y > curr.y)
                    return { src: src, rotation: 0 };
            }
        }
        throw "Error: Invalid snake positions.";
    }
}
function drawSnakeCell(n_cell, snake) {
    if (!indexInGrid(snake[n_cell]))
        return undefined;
    let cell_img = getCellImage(n_cell, snake);
    let grid_tile = getGridTile(snake[n_cell]);
    let margin = 0.05 * grid_tile.size.width;
    return drawImage({
        src: cell_img.src,
        rotation: cell_img.rotation,
        dims: {
            pos: {
                x: grid_tile.pos.x - margin / 2,
                y: grid_tile.pos.y - margin / 2
            },
            size: {
                width: grid_tile.size.width + margin,
                height: grid_tile.size.height + margin
            }
        }
    });
}
function drawSnake(snake) {
    if (snake === undefined || snake === null)
        throw "Error: Snake not provided.";
    if (snake.length < 2)
        throw "Error: Snake length must be 2 or higher.";
    clearSnake();
    const snake_elem = document.getElementById("snake");
    for (let n_cell = 0; n_cell < snake.length; n_cell++) {
        const img = drawSnakeCell(n_cell, snake);
        if (img !== undefined) {
            snake_elem.append(img);
            PAGE.snake_elems.push(img);
        }
    }
}
function getElementDims(elem) {
    let rect = elem.getBoundingClientRect();
    return {
        pos: {
            x: rect.left,
            y: rect.top
        },
        size: {
            width: rect.right - rect.left,
            height: rect.bottom - rect.top
        }
    };
}
function setButtonInCenter(cls) {
    const button = document.createElement("button");
    document.getElementById("button").append(button);
    let grid = getElementDims(document.getElementById("grid"));
    button.className = cls;
    button.style.fontSize = "10em";
    let button_dims = getElementDims(button);
    button.style.marginTop = `${grid.pos.y + (grid.size.height - button_dims.size.height) / 2}px`;
    button.style.marginLeft = `${grid.pos.x + (grid.size.width - button_dims.size.width) / 2}px`;
    return button;
}
function setStartButton() {
    const button = setButtonInCenter("play_button");
    button.textContent = "Start";
    button.onclick = () => {
        button.remove();
        play();
    };
}
function setDeathButton() {
    const button = setButtonInCenter("death_button");
    button.textContent = "Restart!";
    button.onclick = () => {
        button.remove();
        play();
    };
}
function resetPage() {
    for (const bit of PAGE.bits)
        bit[1].remove();
    PAGE.bits = [];
}
function death() {
    resetPage();
    setDeathButton();
}
function setGame() {
    makeGrid(16, 16);
    document.addEventListener('keydown', (event) => {
        let arrow_pos = event.key.indexOf("Arrow");
        if (arrow_pos === -1)
            return;
        let key_str = event.key.substring(arrow_pos + 5).toLowerCase();
        if (!isDirKey(key_str))
            return;
        PAGE.events.key_down[key_str] = true;
    });
    setStartButton();
}
function resetGame() {
    PAGE.snake = [
        { x: 4, y: 4 },
        { x: 3, y: 4 },
        { x: 2, y: 4 },
        { x: 1, y: 4 },
    ];
    drawSnake(PAGE.snake);
    setNewBit(getRandomBitPosition());
    PAGE.is_dead = false;
}
function sleep(ms) {
    return __awaiter(this, void 0, void 0, function* () {
        return new Promise(resolve => setTimeout(resolve, ms));
    });
}
function clearFrameDirections() {
    PAGE.events.key_down.right = false;
    PAGE.events.key_down.left = false;
    PAGE.events.key_down.down = false;
    PAGE.events.key_down.up = false;
}
function getFrameDirection() {
    const right = PAGE.events.key_down.right;
    const left = PAGE.events.key_down.left;
    const down = PAGE.events.key_down.down;
    const up = PAGE.events.key_down.up;
    let default_dir = PAGE.getSnakeDir();
    if (Number(up) + Number(down) + Number(left) + Number(right) !== 1) {
        return default_dir;
    }
    let key_dir;
    if (right)
        key_dir = Direction.right;
    if (left)
        key_dir = Direction.left;
    if (down)
        key_dir = Direction.down;
    if (up)
        key_dir = Direction.up;
    return areOpposite(key_dir, default_dir)
        ? default_dir
        : key_dir;
}
function checkDeath() {
    for (let n1 = 0; n1 < PAGE.snake.length - 1; n1++) {
        for (let n2 = n1 + 4; n2 < PAGE.snake.length; n2 += 2) {
            if (indexEqual(PAGE.snake[n1], PAGE.snake[n2])) {
                PAGE.is_dead = true;
                return;
            }
        }
    }
    if (!indexInGrid(PAGE.snake[0])) {
        PAGE.is_dead = true;
        return;
    }
}
function advanceSnake() {
    let dir = getFrameDirection();
    let pos = copyIndex(PAGE.snake[0]);
    switch (dir) {
        case (Direction.right):
            pos.x++;
            break;
        case (Direction.left):
            pos.x--;
            break;
        case (Direction.down):
            pos.y++;
            break;
        case (Direction.up):
            pos.y--;
            break;
    }
    PAGE.snake.splice(0, 0, pos);
    if (PAGE.grow_snake) {
        PAGE.grow_snake = false;
    }
    else {
        PAGE.snake.pop();
    }
    checkDeath();
}
function listHasIndex(list, index) {
    for (let idx of list) {
        if (indexEqual(idx, index))
            return true;
    }
    return false;
}
function listHasBit(list, index) {
    for (let [idx, _] of list) {
        if (indexEqual(idx, index))
            return true;
    }
    return false;
}
function drawBit(pos) {
    const bits_elem = document.getElementById("bits");
    const elem = drawImage({
        src: "media/bit.png",
        rotation: 0,
        dims: getGridTile(pos)
    });
    bits_elem.append(elem);
    return elem;
}
function setNewBit(pos) {
    PAGE.bits.push([pos, drawBit(pos)]);
}
function getRandomBitPosition() {
    let positions = [];
    for (let y = 0; y < PAGE.rows; y++) {
        for (let x = 0; x < PAGE.cols; x++) {
            let index = { x: x, y: y };
            if (listHasIndex(PAGE.snake, index))
                continue;
            if (listHasBit(PAGE.bits, index))
                continue;
            positions.push(index);
        }
    }
    return positions[Math.floor(Math.random() * positions.length)];
}
function advanceCurrentBits() {
    let head = PAGE.snake[0];
    let bit_found = undefined;
    for (const [n_bit, bit] of PAGE.bits.entries()) {
        if (indexEqual(head, bit[0])) {
            PAGE.grow_snake = true;
            bit[1].remove();
            bit_found = n_bit;
            setNewBit(getRandomBitPosition());
            break;
        }
    }
    if (bit_found !== undefined)
        PAGE.bits.splice(bit_found, 1);
}
function advanceBits() {
    advanceCurrentBits();
}
function play() {
    return __awaiter(this, void 0, void 0, function* () {
        resetGame();
        while (!PAGE.is_dead) {
            yield sleep(200);
            advanceSnake();
            drawSnake(PAGE.snake);
            advanceBits();
            clearFrameDirections();
            PAGE.frame++;
        }
        death();
    });
}
function main() {
    setGame();
}
main();
//# sourceMappingURL=index.js.map