package main
import (
	"fmt"
	"os"
	"io/ioutil"
	"strings"
	"strconv"
)

func main() {

  inp := read_input()

  sol1 := part1(inp)
  fmt.Println("part1:",sol1)

  sol2 := part2(inp)
  fmt.Println("part2:", sol2)
} 

func part1(games []Game) int {
	condition := [3]int{12, 13, 14}
	total := 0

	for i := 0; i < len(games); i++ {
		invalid := false
		for j := 0; j < len(games[i].rounds); j++ {
			for k := 0; k < 3; k++ {
				if games[i].rounds[j][k] > condition[k] {
					invalid = true
				}
			}
		}
		if !invalid {
			total += games[i].id
		}
	}
	return total
}
func part2(games []Game) int {
	total := 0

	for i := 0; i < len(games); i++ {
		min_cubes := make([]int, 3)
		for j := 0; j < len(games[i].rounds); j++ {
			for k := 0; k < 3; k++ {
				if games[i].rounds[j][k] > min_cubes[k] {
					min_cubes[k] = games[i].rounds[j][k]
				}
			}
		}
		total += min_cubes[0] * min_cubes[1] * min_cubes[2]
		
	}
	return total
}
type Game struct {
	id		int
	rounds	[][]int
}

const (

	Red int = iota
	Green
	Blue
)

func read_input() []Game {
	file, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Println("Cannot read the file")
		os.Exit(1)
	}

	file_str := string(file)

	lines := strings.Split(file_str, "\n")

	parsed_games := make([]Game, len(lines))

	for i := 0; i < len(lines); i++ {
		first_split := strings.Split(lines[i], ": ")

		game_number := strings.Split(first_split[0], " ")[1]
		
		game_num, err := strconv.Atoi(game_number)
		if err != nil {
			fmt.Println("failed to convert!")
			os.Exit(1)
		}
		rounds := strings.Split(first_split[1], "; ")

		parsed_rounds := make([][]int, len(rounds))

		for j := 0; j < len(rounds); j++ {
			round := make([]int, 3)
			colors := strings.Split(rounds[j], ", ")
			for k := 0; k < len(colors); k++ {
				parts := strings.Split(colors[k]," ")
				num, err := strconv.Atoi(parts[0])
				if err != nil {
					fmt.Println("failed to convert!")
					os.Exit(1)
				}
				var color int
				switch parts[1] {
				case "blue":
					color = Blue
				case "red":
					color = Red
				case "green":
					color = Green
				}
				round[color] = num
			}
			parsed_rounds[j] = round
		}
		parsed_games[i] = Game{game_num, parsed_rounds}
	}
	return parsed_games
}