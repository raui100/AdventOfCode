const text: string = await Deno.readTextFile("04.txt", "utf8");

let numbers: Array<Array<number>> = text.split("\n")
  .map((line) => line.replace(",", "-").split("-").map(Number));

console.log(
  numbers
    .filter((nums) => (nums[0] - nums[2]) * (nums[1] - nums[3]) <= 0)
    .length,
); // 547
console.log(
  numbers
    .filter((nums) => !(nums[0] > nums[3] || nums[1] < nums[2]))  // Possible filter
    .filter((nums) => Math.max(nums[0], nums[2]) <= Math.min(nums[1], nums[3]))  // Alternative filter
    .length,
); // 843
