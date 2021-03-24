function parseInput(input: string) {
  return input.split("\n").map((num) => parseInt(num));
}

let joltageRatings: number[] = parseInput(await Deno.readTextFile('./inputs/day10.txt'));

function part1(joltageRatings: number[]) {
  let outlet = 0;
  let diff1 = 0;
  let diff3 = 1; // bc of the highest rated joltage rule

  joltageRatings.sort((a: number, b: number) => a - b);
  joltageRatings.forEach((rating: number) => {
    if (outlet + 1 === rating) {
      diff1 += 1;
    } else if (outlet + 3 === rating) {
      diff3 += 1;
    }

    outlet = rating;
  });
  return diff1;
}

console.log(part1(joltageRatings));
