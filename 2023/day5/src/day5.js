import * as fs from 'fs'
import { createMappings, mappingIterator } from './utils.js'

const main = () => {
  // read input from file
  const data = fs.readFileSync('input2.txt', 'utf8');
  const lines = data.split('\n');

  let ranges = lines[0].split(":")[1]
  .trim()
  .split(" ")
  .filter(x => x.trim() !== "")
  .map(x => parseInt(x))
  .reduce((acc, curr, index, array) => {
      if (index % 2 === 0) {
          acc.push([curr, array[index + 1]]);
      }
      return acc;
  }, []);

  const mappings = createMappings(lines);
  console.info("mappings", JSON.stringify(mappings))

  const result = mappingIterator(ranges, mappings)

  console.log("result", result.sort((a, b) => a[0] - b[0])[0][0])
}

// (5, 30) -> (6, 4), (10, 11), (21, 9), (30, 1)
// (5, 30) -> (6, 4), (50, 11), (10, 9), (30, 1)

// [0] destination, [1] source, [2] length
main()
