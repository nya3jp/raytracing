package renderer

import (
	"fmt"
	"os"
	"sync"
	"sync/atomic"
)

type queue struct {
	limit int
	jobs  []func()
}

func newQueue(limit int) *queue {
	return &queue{limit: limit}
}

func (q *queue) Add(job func()) {
	q.jobs = append(q.jobs, job)
}

func (q *queue) Run() {
	tokens := make(chan struct{}, q.limit)
	jobs := q.jobs
	q.jobs = nil

	var wg sync.WaitGroup
	wg.Add(len(jobs))
	var done int64

	for _, job := range jobs {
		job := job
		go func() {
			tokens <- struct{}{}
			defer func() {
				<-tokens
				newDone := atomic.AddInt64(&done, 1)
				fmt.Fprintf(os.Stderr, "%d/%d\r", newDone, len(jobs))
				wg.Done()
			}()
			job()
		}()
	}

	wg.Wait()
}
