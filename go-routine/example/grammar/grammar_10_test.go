package grammar

import (
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func TestGrammar10_00(t *testing.T) {
	mutex := sync.Mutex{}
	mutex.Lock()
	mutex.Unlock()

	// 正常模式（非公平锁）正常模式下：
	// 所有等待锁的goroutine按照FIFO（先进先出）顺序等待。
	// 唤醒的goroutine不会直接拥有锁，而是会和新请求goroutine竞争锁。
	// 新请求的goroutine更容易抢占：因为它正在CPU上执行，所以刚刚唤醒的goroutine有很大可能在锁竞争中失败。
	// 在这种情况下，这个被唤醒的goroutine会加入到等待队列的前面

	// 饥饿模式（公平锁）为了解决了等待goroutine队列的长尾问题饥饿模式下，直接由unlock把锁交给等待队列中排在第一位的goroutine (队头)，
	// 同时，饥饿模式下，新进来的goroutine不会参与抢锁也不会进入自旋状态，会直接进入等待队列的尾部。这样很好的解决了老的goroutine一直抢不到锁的场景。
	// 饥饿模式的触发条件：当一个goroutine等待锁时间超过1毫秒时，或者当前队列只剩下一个goroutine的时候，Mutex切换到饥饿模式。

	// 对于两种模式，正常模式下的性能是最好的，goroutine可以连续多次获取锁，
	// 饥饿模式解决了取锁公平的问题，但是性能会下降，这其实是性能和公平的一个平衡模式
}

func TestGrammar10_01(t *testing.T) {
	mu1 := &sync.Mutex{}
	mu1.Lock()
	mu2 := mu1
	mu2.Unlock()
	mu1.Unlock()
}

func TestGrammar10_02(t *testing.T) {
	rw := sync.RWMutex{}
	rw.Lock()
	rw.Unlock()
}

func TestGrammar10_03(t *testing.T) {
	// Cond实现了一种条件变量，可以使用在多个Goroutine等待共享资源ready的场景
	cond := sync.NewCond(&sync.Mutex{})
	cond.L.Lock()

	cond.Wait()
	cond.Broadcast() // 唤醒所有
	cond.Signal()    // 唤醒一个

	cond.L.Unlock()
}

func TestGrammar10_04(t *testing.T) {
	wg := sync.WaitGroup{}
	wg.Add(10)
	for i := 0; i < 10; i++ {
		go func() {
			time.Sleep(time.Second * 2)
			println("----")
			wg.Done()
		}()
	}
	wg.Wait()
	println("wg done!")
}

func TestGrammar10_05(t *testing.T) {
	once := sync.Once{}
	for i := 0; i < 10; i++ {
		once.Do(func() {
			println("sync once demo")
		})
	}
}

func TestGrammar10_06(t *testing.T) {
	type App struct {
		Name string
	}
	v := atomic.Value{}
	v.Store(App{Name: "app"})
	val := v.Load().(App)
	println(val.Name)
}

func TestGrammar10_07(t *testing.T) {
	type App struct {
		Name string
	}
	pool := sync.Pool{
		New: func() any {
			return new(App)
		},
	}
	_ = pool.Get().(App)
}
