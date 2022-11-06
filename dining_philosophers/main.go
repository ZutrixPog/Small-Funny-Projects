package main

import "dining/dining"

func main() {
	table := dining.NewTable(5)
	table.StartDining()
}
