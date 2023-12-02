import { readFileSync } from "fs";

export const part1 = (filepath) => {
	const content = readFileSync(filepath, "utf-8");

	const maxAllowed = {
		red: 12,
		green: 13,
		blue: 14,
	};

	let ans = content
		.trim()
		.split("\n")
		.map((line) => {
			const parts = line.split(":");
			const id = +parts[0].split(" ")[1];

			const res = parts[1]
				.split(";")
				.map((part) =>
					part
						.trim()
						.split(",")
						.map((ball) => ball.trim().split(" "))
						.some(([count, color]) => maxAllowed[color] < +count)
				)
				.some((ele) => ele);

			if (!res) return id;
			return 0;
		})
		.reduce((acc, curr) => acc + curr, 0);

	console.log(ans);
};

export const part2 = (filepath) => {
	const content = readFileSync(filepath, "utf-8");

	let ans = content
		.trim()
		.split("\n")
		.map((line) => {
			const res = line
				.split(":")[1]
				.split(";")
				.map((part) =>
					part
						.trim()
						.split(",")
						.map((ball) => ball.trim().split(" "))
						.reduce(
							(acc, [count, color]) => ({
								...acc,
								[color]: +count,
							}),
							{}
						)
				)
				.reduce(
					(acc, d) => ({
						red: Math.max(acc.red, d.red || 0),
						green: Math.max(acc.green, d.green || 0),
						blue: Math.max(acc.blue, d.blue || 0),
					}),
					{
						red: 0,
						green: 0,
						blue: 0,
					}
				);
			return res.red * res.green * res.blue;
		})
		.reduce((acc, curr) => acc + curr, 0);

	console.log(ans);
};
