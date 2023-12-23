import { describe, test, expect } from "vitest"
import { splitToThree, rangeSplitter, calculateOverlap, mappingIterator } from "./utils"

describe('utils', () => {
  describe('mappingIterator', () => {
    const mappings = [
      {"name":"seed-to-soil map","values":[[50,98,2],[52,50,48]]},
      {"name":"soil-to-fertilizer map","values":[[0,15,37],[37,52,2],[39,0,15]]},
      {"name":"fertilizer-to-water map","values":[[49,53,8],[0,11,42],[42,0,7],[57,7,4]]},
      {"name":"water-to-light map","values":[[88,18,7],[18,25,70]]},
      {"name":"light-to-temperature map","values":[[45,77,23],[81,45,19],[68,64,13]]},
      {"name":"temperature-to-humidity map","values":[[0,69,1],[1,0,69]]},
      {"name":"humidity-to-location map","values":[[60,56,37],[56,93,4]]}
    ]
    test('one seed', () => {
      const heroRanges = [[82, 1]]
      expect(mappingIterator(heroRanges, mappings)).toEqual([[46, 1]])
    })

    test.skip('multiple seeds', () => {
      const heroRanges = [[79, 14], [55, 13]]
      expect(mappingIterator(heroRanges, mappings)).toEqual([[46, 1], [82, 3]])
    })
  })

  describe('rangeSplitter', () => {
    test('test1', () => {
      const range = [0, 1]
      const mapping = { values: [[16, 0, 10]] }

      expect(rangeSplitter(range, mapping)).toEqual([[16, 1]])
    })

    test('test2', () => {
      const range = [0, 11]
      const mapping = { values: [[30, 5, 3]] }
      expect(rangeSplitter(range, mapping)).toEqual([[0, 5], [30, 3], [8, 3]])
    })

    test('[5, 25] -> [[5, 5], [50, 11], [10, 9], [30, 1]', () => {
      const range = [5, 26]
      const mapping = {
        values: [
          [50, 10, 11], [10, 21, 9]
        ]
      }

      expect(rangeSplitter(range, mapping)).toEqual([[5, 5], [50, 11], [10, 9], [30, 1]])
    })

    test('test4', () => {
      const range = [82, 1]
      const mapping = {
        values: [
          [52, 50, 48],
          [50, 98, 2]
        ]
      }

      expect(rangeSplitter(range, mapping)).toEqual([[84, 1]])
    })

    test('test5', () => {
      const range = [74, 4]
      const mapping = {
        values: [
        [ 81, 45, 19 ],
        [ 68, 64, 13 ],
        [ 45, 77, 23 ]]
      }

      expect(rangeSplitter(range, mapping)).toEqual([[78, 3], [45, 1]])
    })

    describe('hard tests', () => {
      test('hard 1 simplified', () => {
        const range = [835_041_333, 194_256_900]
        const mapping = { values: [
          [ 2_938_601_691, 473_979_048, 463_977_619 ],
          [ 1_812_605_508, 937_956_667, 271_194_541 ],
        ]}

        // 473_979_048 + 463_977_619 = 937_956_667
        // 937_956_667 - 835_041_333 = 102_915_334
        expect(rangeSplitter(range, mapping)).toEqual([[3_299_663_976, 102_915_334], [1_812_605_508, 91_341_566]])

        // Tries to push: [835_041_333, 102_915_334]

        // split.first[0] = 835_041_333
        // mapping.values[i][0] = 1_812_605_508
      })
    })
  })

  describe('splitToThree', () => {
    test.each`
    heroRanges  | values         | expected
    ${[15, 10]} | ${[80, 20, 4]} | ${{ first: [15, 5], second: [20, 4], third: [24, 1] }}
    ${[0, 100]} | ${[80, 20, 4]} | ${{ first: [0, 20], second: [20, 4], third: [24, 76] }}
    ${[0, 10]}  | ${[80, 20, 4]} | ${{ first: [0, 10] }}
    ${[21, 2]}  | ${[80, 20, 4]} | ${{ second: [21, 2] }}
    ${[50, 10]} | ${[80, 20, 4]} | ${{ third: [50, 10] }}
    `('range $heroRanges, values $values -> $expected', ({ heroRanges, values, expected }) => {
      expect(splitToThree(heroRanges, values)).toEqual(expected)
    })
  })

  describe('utilities', () => {
    describe('calculateOverlap', () => {
      test('should return undefined when no overlap', () => {
        expect(calculateOverlap([0, 1], [1, 1])).toEqual(undefined)
      })

      test('should return range when they are same', () => {
        expect(calculateOverlap([15, 15], [15, 15])).toEqual([15, 15])
      })

      test('should calculate overlap when there is overlap 1', () => {
        expect(calculateOverlap([10, 5], [14, 5])).toEqual([14, 1])
      })

      test('should calculate overlap when there is overlap 2', () => {
        expect(calculateOverlap([10, 5], [6, 5])).toEqual([10, 1])
      })
    })
  })
})

// [0] destination, [1] source, [2] length
