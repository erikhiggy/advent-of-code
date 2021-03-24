function parseInput(input: string) {
  return input.split("\n").map((num) => parseInt(num));
}

let joltageRatings: number[] = parseInput(await Deno.readTextFile('./inputs/day10.txt'));
joltageRatings.unshift(0);
joltageRatings.sort((a: number, b: number) => a - b);

function part1(joltageRatings: number[]) {
  let outlet = 0;
  let diff1 = 0;
  let diff3 = 1; // bc of the highest rated joltage rule

  joltageRatings.forEach((rating: number) => {
    if (outlet + 1 === rating) {
      diff1 += 1;
    } else if (outlet + 3 === rating) {
      diff3 += 1;
    }

    outlet = rating;
  });
  return diff1 * diff3;
}

let valueMap = new Map();

function part2(num: number) {
  if (num === (joltageRatings.length - 1)) return 1;
  if (valueMap.has(num)) {
    // console.log(valueMap.get(num));
    return valueMap.get(num);
  }

  let ans = 0;
  let j = num + 1;
  for (; j < joltageRatings.length; j++) {
    if (joltageRatings[j] - joltageRatings[num] > 3) break;
    ans += part2(j);
  }
  valueMap.set(num, ans);
  return ans;
}

console.log(part1(joltageRatings));
console.log(part2(0));
