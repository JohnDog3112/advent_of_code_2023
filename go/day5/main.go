package main
import (
	"fmt"
	"os"
	"io/ioutil"
	"strings"
	"strconv"
	"sort"
)

func main() {

  inp := read_input();

  //fmt.Println(inp);

  sol1 := part1(inp)
  fmt.Println("part1:",sol1)

  sol2 := part2(inp)
  fmt.Println("part2:", sol2)
} 

func part1(almanac Almanac) int {

	min_location := 99999999999999999;
	for i := 0; i < len(almanac.seeds); i++ {
		location := almanac.seed_to_location(almanac.seeds[i]);
		if location < min_location {
			min_location = location;
		}
	}
	return min_location;
}
func part2(almanac Almanac) int {
	seed_ranges := make([]Range, len(almanac.seeds)/2);
	for i := 0; i < len(almanac.seeds)/2; i++ {
		seed_ranges[i] = Range {
			almanac.seeds[i*2],
			almanac.seeds[i*2] + almanac.seeds[i*2+1]-1,
		};
	}
	//fmt.Println(seed_ranges);

	var ranges_to_map []Range = seed_ranges;
	var mapped_ranges []Range;
	for i := 0; i < len(almanac.maps); i++ {
		t_map := almanac.maps[i].ranges;

		for len(ranges_to_map) > 0 {
			to_map := &ranges_to_map[0];
			ranges_to_map = ranges_to_map[1:];
			for j := 0; j < len(t_map); j++ {
				before, middle, after := t_map[j].split_range(*to_map);
				//fmt.Println("&&&&&&&&&&&&&&");
				//fmt.Println(before, middle, after);

				if before != nil {
					//maps are sorted by source_start
					//so anything before doesn't have a range
					mapped_ranges = append(mapped_ranges, *before);
				}
				if middle != nil {
					//this has a range, map it
					mapped_ranges = append(mapped_ranges, t_map[j].map_range(*middle));
				}
				if after != nil {
					//needs to be checked with next range
					to_map = after;
				} else {
					to_map = nil;
					break;
				}
				
			}
			//anything at the end doesn't have a range either
			if to_map != nil {
				mapped_ranges = append(mapped_ranges, *to_map);
			}
		}
		//fmt.Println("---------------");
		//fmt.Println(mapped_ranges);

		ranges_to_map = mapped_ranges;
		mapped_ranges = []Range{};

	}
	//fmt.Println(ranges_to_map);

	min_loc := 99999999999999999;
	for i := 0; i < len(ranges_to_map); i++ {
		if ranges_to_map[i].start < min_loc {
			min_loc = ranges_to_map[i].start;
		}
	}
	return min_loc;
}

type Range struct {
	start	int
	end		int
}
func (m MapLayer) split_range(inp Range) (*Range, *Range, *Range) {
	if inp.end < m.source_start {
		return &inp, nil, nil;
	}
	if inp.start > m.source_end {
		return nil, nil, &inp;
	}

	start := inp.start;
	end	  := inp.end;

	var before_range *Range;
	if start < m.source_start {
		before_range = &Range {
			start,
			m.source_start-1,
		};
		start = m.source_start;
	}

	var after_range *Range;
	if end > m.source_end {
		after_range = &Range {
			m.source_end+1,
			end,
		};
		end = m.source_end;
	}

	var actual_range *Range;
	actual_range = &Range {
		start,
		end,
	};

	return before_range, actual_range, after_range;
}
func (m MapLayer) map_range(inp Range) Range {
	return Range {
		inp.start - m.source_start + m.destination_start,
		inp.end - m.source_start + m.destination_start,
	};
}
type Almanac struct {
	seeds		[]int
	maps		[]Map
}
type Map struct {
	ranges		[]MapLayer
}
type MapLayer struct {
	destination_start	int
	source_start		int
	range_length		int
	source_end			int
}
func (a Almanac) seed_to_location(seed int) int {
	prev_val := seed;
	for i := 0; i < len(a.maps); i++ {
		prev_val = a.maps[i].map_inp(prev_val);
	}
	return prev_val;
}
func (m Map) map_inp(inp int) int {
	for i := 0; i < len(m.ranges); i++ {
		if m.ranges[i].inp_in_range(inp) {
			return m.ranges[i].map_inp(inp);
		}
	}
	//no match in maps
	return inp;
}

func (m MapLayer) inp_in_range(inp int) bool {
	return inp >= m.source_start && inp <= m.source_end;
}
func (m MapLayer) map_inp(inp int) int {
	return inp-m.source_start + m.destination_start
}

func read_input() Almanac{
	file, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Println("Cannot read the file")
		os.Exit(1)
	}

	file_str := string(file)

	sections := strings.Split(file_str, "\n\n")

	/*for i := 0; i < len(sections); i++ {
		fmt.Println("-------------");
		fmt.Println(sections[i]);
	}*/

	//seeds
	seed_part_one := strings.Split(sections[0], ": ")[1];
	seed_list := strings.Split(seed_part_one, " ");
	
	seeds := make([]int, len(seed_list));
	for i := 0; i < len(seed_list); i++ {
		seed_num, err := strconv.Atoi(seed_list[i])
		if err != nil {
			fmt.Println("failed to convert!")
			os.Exit(1)
		}
		seeds[i] = seed_num;
	}

	//maps
	maps := make([]Map, len(sections)-1)

	for i := 1; i < len(sections); i++ {
		map_layer_part_one := strings.Split(sections[i], ":\n")[1]
		map_layer_lines := strings.Split(map_layer_part_one, "\n")

		map_layer := make([]MapLayer, len(map_layer_lines))

		for j := 0; j < len(map_layer); j++ {
			components := strings.Split(map_layer_lines[j]," ");
			parsed_components := make([]int, 3)
			for k := 0; k < 3; k++ {
				parse_num, err := strconv.Atoi(components[k])
				if err != nil {
					fmt.Println("failed to convert!")
					os.Exit(1)
				}
				parsed_components[k] = parse_num;
			}
			map_layer[j] = MapLayer {
				parsed_components[0],
				parsed_components[1],
				parsed_components[2],
				parsed_components[1] + parsed_components[2]-1,
			};
		}

		//sort by source_start
		sort.Slice(map_layer, func(i, j int) bool {
			return map_layer[i].source_start < map_layer[j].source_start;
		})
		maps[i-1] = Map {
			map_layer,
		};
	}
	//fmt.Println(maps)

	return Almanac {
		seeds,
		maps,
	};
}