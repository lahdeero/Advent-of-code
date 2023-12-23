export const createMappings = (lines) => {
  let collect = false;
  let mappings = [];
  let currentMapping = null;

  const stringNumbersToNumbers = (line) => {
    return line.trim().split(" ").map(x => parseInt(x));
  }

  for (const line of lines) {
    if (line.includes("map:")) {
        collect = true;
        const name = line.split(":")[0].trim();
        currentMapping = {
            name: name,
            values: [],
        };
        mappings.push(currentMapping);
    } else if (line.trim() === "") {
        collect = false;
    } else if (collect && currentMapping) {
        currentMapping.values.push(stringNumbersToNumbers(line));
    }
  }
  return mappings;
}

export const calculateOverlap = (range1, range2) => {
  const start = Math.max(range1[0], range2[0]);
  const end = Math.min(range1[0] + range1[1], range2[0] + range2[1]);
  return start < end ? [start, end - start] : undefined
}

export const mappingIterator = (heroRanges, mappings) => {
  for (const mapping of mappings) {
    console.debug("F*****************F")
    mapping.values.sort((a, b) => a[1] - b[1])
    console.debug("mapping", mapping.name)
    console.debug("heroRanges", heroRanges)
    let new_ranges = []
    for (let i = 0; i < heroRanges.length; i++) {
      console.debug("A------------------A")
      const heroRange = heroRanges[i]
      console.debug("heroRange", heroRange)
      console.debug("mapping", mapping)
      const split = rangeSplitter(heroRange, mapping)
      console.debug("välitulos", split)
      new_ranges.push(...split)
      // const new_range_total_length = new_ranges.reduce((acc, curr) => acc + curr[1], 0)
      // Should probably combine or filter hero ranges
      console.debug("E------------------E")
    }
    heroRanges = [...new_ranges]
    console.debug("S*****************S\n")
  }

  return heroRanges
}

export const rangeSplitter = (hero, mapping) => {
  let resultRanges = []
  let overflow = undefined
  let firstPushed = false // TODO: is this needed?
  let remaining = hero[1]
  for (let i = 0; i < mapping.values.length; i++) {
    const split = splitToThree(hero, mapping.values[i])
    // console.debug("split", split)
    if (!firstPushed && split.first && split.first[0] < mapping.values[i][0]) {
      resultRanges.push(split.first)

      console.log("\nXXXXXXXXXXX")
      console.log("i", i)
      console.log("remaining", remaining)
      console.log("split", split)
      console.log("hero", hero)
      console.log("mapping", mapping)
      console.log("resultRanges", resultRanges)
      console.log("YYYYYYYYYYYYYY\n")
      1+1
      remaining = calculateRemaining(remaining, split.first[1])
      firstPushed = true
    }
    if (remaining > 0 && split.second) {
      const starting = Math.max(hero[0] + mapping.values[i][0] - mapping.values[i][1], mapping.values[i][0])
      resultRanges.push([starting, split.second[1]])
      remaining = calculateRemaining(remaining, split.second[1])
      if (remaining === 0) {
        break
      }
    }
    overflow = split.third
  }
  overflow && remaining > 0 && resultRanges.push(overflow)
  return resultRanges
}

// [15, 10], [X, 20, 4] -> [[15, 5], [20, 4], [24, 1]]
export const splitToThree = (hero, values) => {
  if (!Array.isArray(values) || values.length > 3) {
    throw new Error(`Invalid values: ${values}`)
  }
  if (!Array.isArray(hero)) {
    throw new Error(`Invalid range: ${hero}`)
  }

  let remaining = hero[1]
  let first = hero[0] < values[1] ? [hero[0], Math.min(hero[1], values[1] - hero[0])] : undefined
  remaining -= first ? first[1] : 0
  let second = calculateOverlap(hero, [values[1], values[2]])
  remaining -= second ? second[1] : 0
  let third = remaining > 0 && [Math.max(hero[0], values[1] + values[2]), remaining]
  if (third[0] !== undefined && (isNaN(third[0]) || isNaN(third[1]))) {
    console.debug("third[0]", third[0])
    console.debug("third[1]", third[1])
    console.debug("range[0]", hero[0])
    console.debug("range[1]", hero[1])
    console.debug("values[1]", values[1])
    console.debug("values[2]", values[2])
    console.debug("remaining", remaining)
    throw new Error()
  }
  if (third && !Array.isArray(third)) {
    throw new Error(`Third is not array: ${third}`)
  }
  return {
    first: !!first ? first : undefined,
    second: !!second ? second : undefined,
    third: !!third ? third : undefined
  }
}

const calculateRemaining = (current, toReduce) => {
  if (!toReduce) {
    return current
  }
  const ret = current - toReduce
  if (ret < 0) {
    throw new Error(`Reducing more than current from remaining ${current} - ${toReduce}`)
  }
  return ret
}
