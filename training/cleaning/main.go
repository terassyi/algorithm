package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Step interface {
	solve() (int, error)
}

type Step1 struct {
	step  int
	rooms []Room
}

type Room struct {
	number         int
	cluttered      int
	homeAppliances []HomeAppliance
}

func (room *Room) clean() error {

}

func newRoom(n, cluttered int, apps []HomeAppliance) Room {
	return Room{
		number:         n,
		cluttered:      cluttered,
		homeAppliances: apps,
	}
}

type HomeAppliance struct {
	time       int
	efficiency int
}

func newHomeAppliance(t, s int) HomeAppliance {
	return HomeAppliance{
		time:       t,
		efficiency: s,
	}
}

func main() {
	step1, err := step1Input()
	if err != nil {
		panic(err)
	}
	fmt.Printf("number of rooms %d\n", len(step1.rooms))
}

func step1Input() (*Step1, error) {
	lines := getStdin()
	step, err := strconv.Atoi(lines[0])
	if err != nil {
		return nil, err
	}
	if step != 1 {
		return nil, fmt.Errorf("Not Step 1.")
	}
	n, err := strconv.Atoi(lines[1])
	if err != nil {
		return nil, err
	}
	splitted := strings.Split(lines[2], " ")
	clutteredList := make([]int, 0, n)
	for _, a := range splitted {
		aa, err := strconv.Atoi(a)
		if err != nil {
			return nil, err
		}
		clutteredList = append(clutteredList, aa)
	}
	ki := 3
	c := 0
	rooms := make([]Room, 0, n)
	for {
		// room
		cluttered := clutteredList[c]
		k, err := strconv.Atoi(lines[ki])
		if err != nil {
			return nil, err
		}
		ki = ki + k + 1
		apps := make([]HomeAppliance, 0, k)
		for i := ki - k; i < ki; i++ {
			tsStr := strings.Split(lines[i], " ")
			t, err := strconv.Atoi(tsStr[0])
			if err != nil {
				return nil, err
			}
			s, err := strconv.Atoi(tsStr[1])
			if err != nil {
				return nil, err
			}
			homeAppliance := newHomeAppliance(t, s)
			apps = append(apps, homeAppliance)
		}
		room := newRoom(c, cluttered, apps)
		rooms = append(rooms, room)
		c++
		if c >= n {
			break
		}
	}
	return &Step1{
		step:  step,
		rooms: rooms,
	}, nil
}

func getStdin() []string {
	stdin, _ := ioutil.ReadAll(os.Stdin)
	return strings.Split(strings.TrimRight(string(stdin), "\n"), "\n")
}
