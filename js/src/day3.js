import { readFileSync } from "fs";

const isDigit = (char) => {
	return char >= "0" && char <= "9";
};

const isSymbol = (char) => {
	return !isDigit(char) && char !== ".";
};

const parseNumbersFromLine = (line, y) => {
	const numbers = [];
	const X = line.length;

	for (let x = 0; x < X; x++) {
		const start = x;
		let curr = 0;
		for (; x < X && isDigit(line[x]); x++) {
			curr = curr * 10 + parseInt(line[x]);
		}
		if (curr > 0) {
			x--;
			numbers.push({
				start,
				end: x,
				y,
				value: curr,
			});
		}
	}

	return numbers;
};

const getValidNumbers = (numbers, x, y) => {
	const validNumbers = [];

	for (const number of numbers) {
		if (number.y === y)
			if (number.start == x + 1 || number.end == x - 1) {
				validNumbers.push(number);
				continue;
			}

		if (number.y === y + 1 || number.y === y - 1)
			if (number.start <= x + 1 && number.end >= x - 1) validNumbers.push(number);
	}

	return validNumbers;
};

const getGear = (numbers, x, y) => {
	const validNumbers = getValidNumbers(numbers, x, y);
	if (validNumbers.length === 2) return validNumbers[0].value * validNumbers[1].value;
	return 0;
};

export const part1 = (path) => {};

export const part2 = (path) => {
	const content = readFileSync(path, "utf8");
	const grid = content.split("\n");
	const Y = grid.length;
	const X = grid[0].length;

	const allNumbers = [];
	for (let y = 0; y < Y; y++) {
		const numbers = parseNumbersFromLine(grid[y], y);
		allNumbers.push(...numbers);
	}

	let sum = 0;
	for (let y = 0; y < Y; y++) {
		for (let x = 0; x < X; x++) {
			if (grid[y][x] == "*") sum += getGear(allNumbers, x, y);
		}
	}

	console.log(sum);
};
