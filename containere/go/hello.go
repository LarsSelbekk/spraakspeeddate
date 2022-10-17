package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func main() {
	bytes, _ := os.ReadFile("kart.txt")
	world := make([][]string, 0, 100)
	json.Unmarshal(bytes, &world)

	height := len(world)
	width := len(world[0])

	// Tree-set structure, two elements are in the same set if they have the same
	// ultimate (recursive) representative, meaning A -> B -> C and D -> C both have
	// C as the top representative, and therefore belong to the same set
	representative := make([]int, width*height)
	for i := range representative {
		representative[i] = -1
	}

	// Traverse the map and add 1's	to islands, joining sets along the way
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			if world[y][x] == "1" {
				// It is an island
				// Set its representative to itself to create a unary set
				representative[y*width+x] = y*width + x
				isLeftIsland := x-1 >= 0 && representative[y*width+x-1] >= 0
				isUpIsland := y-1 >= 0 && representative[(y-1)*width+x] >= 0
				if isLeftIsland && isUpIsland {
					// Join left island with up island
					leftTopRep := getTopRepresentative(y*width+x-1, representative)
					upTopRep := getTopRepresentative((y-1)*width+x, representative)
					if leftTopRep != upTopRep {
						representative[leftTopRep] = upTopRep
					}
					representative[y*width+x] = upTopRep
				} else if isLeftIsland {
					// add to left island
					representative[y*width+x] = representative[y*width+x-1]
				} else if isUpIsland {
					// add to up island
					representative[y*width+x] = representative[(y-1)*width+x]
				}
			}
		}
	}

	// Flatten the representative tree
	for y := 0; y < len(world); y++ {
		for x := 0; x < len(world); x++ {
			representative[y*width+x] = getTopRepresentative(y*width+x, representative)
		}
	}

	// Collect unique top representatives (distinct islands)
	islands := make(map[int]struct{})
	var member struct{}
	for i := range representative {
		islands[representative[i]] = member
	}
	delete(islands, -1) // Remove the undefined representative

	fmt.Println(len(islands))

}

// Get the recursive top representative of a tile
func getTopRepresentative(index int, representative []int) int {
	top := representative[index]
	for top >= 0 && representative[top] != top {
		top = representative[top]
	}
	return top
}
