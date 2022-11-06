package dining

import "sync"

type Table struct {
	phils []*Phil
	forks []*fork
}

func NewTable(philnum int) *Table {
	forks := make([]*fork, philnum)
	for i := 0; i < philnum; i++ {
		forks[i] = new(fork)
	}

	phils := make([]*Phil, philnum)
	for i := 0; i < philnum; i++ {
		phils[i] = NewPhil(i, forks[i], forks[(i+1)%philnum])
	}

	return &Table{
		phils: phils,
		forks: forks,
	}
}

func (t *Table) StartDining() {
	var wg sync.WaitGroup

	for i, _ := range t.phils {
		wg.Add(1)
		go func(i int) {
			for {
				t.phils[i].Dine()
			}
			wg.Done()
		}(i)
	}
	wg.Wait()
}
