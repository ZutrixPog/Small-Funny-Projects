package dining

import (
	"testing"
)

func TestDining(t *testing.T) {
	table := NewTable(5)
	table.StartDining()
}
