package dining

import (
	"fmt"
	"math"
	"math/rand"
	"sync"
	"time"
)

type fork struct{ sync.Mutex }

type Phil struct {
	id                  int
	LeftFork, RightFork *fork
}

func NewPhil(id int, left *fork, right *fork) *Phil {
	return &Phil{
		id:        id,
		LeftFork:  left,
		RightFork: right,
	}
}

func (p *Phil) Dine() {
	for {
		p.pickFork()
		p.eat()
		p.putFork()
		p.Think()
	}
}

func (p *Phil) pickFork() {
	p.LeftFork.Lock()
	p.RightFork.Lock()
}

func (p *Phil) putFork() {
	p.LeftFork.Unlock()
	p.RightFork.Unlock()
}

func (p *Phil) eat() {
	fmt.Printf("Philosopher %d is eating! \n", p.id+1)
	time.Sleep(time.Duration(getRandom(1, 5)) * time.Second)
}

func (p *Phil) Think() {
	fmt.Printf("Philosopher %d is thinking! \n", p.id+1)
	time.Sleep(time.Duration(getRandom(1, 5)) * time.Second)
}

func getRandom(min uint32, max uint32) uint32 {
	rand.Seed(time.Now().UnixNano())
	random := rand.Uint32()
	return random/math.MaxUint32*(max-min) + min
}
