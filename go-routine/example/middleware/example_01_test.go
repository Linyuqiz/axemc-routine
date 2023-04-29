package common

import (
	"context"
	"fmt"
	"github.com/go-redis/redis/v8"
	"testing"
	"time"
)

func TestExample01_00(t *testing.T) {
	// Redis 集群没有使用一致性hash,而是引入了哈希槽的概念，Redis 集群有16384 个哈希槽，
	// 每个key 通过CRC16 校验后对16384 取模来决定放置哪个槽，集群的每个节点负责一部分hash槽
	rdb := redis.NewClient(&redis.Options{
		Addr:     ":6379",
		Password: "", // no password set
		DB:       0,  // use default DB
		PoolSize: 10, // 连接池大小
	})
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	r, err := rdb.Ping(ctx).Result()
	if err != nil {
		panic(err)
	}
	println("redis connection success: ", r)

	err = rdb.Set(ctx, "score", 100, 0).Err()
	if err != nil {
		fmt.Printf("set score failed, err:%v\n", err)
		return
	}
	val, err := rdb.Get(ctx, "score").Result()
	if err != nil {
		fmt.Printf("get score failed, err:%v\n", err)
		return
	}
	fmt.Println("key score value: ", val)

	zk := "language_rank"
	lan := []*redis.Z{
		{Score: 90.0, Member: "Golang"},
		{Score: 98.0, Member: "Java"},
		{Score: 95.0, Member: "Python"},
		{Score: 97.0, Member: "JavaScript"},
		{Score: 99.0, Member: "C/C++"},
	}
	num, err := rdb.ZAdd(ctx, zk, lan...).Result()
	if err != nil {
		fmt.Printf("zadd failed, err:%v\n", err)
		return
	}
	fmt.Printf("zadd %d succ.\n", num)
}

func TestExample01_01(t *testing.T) {
	rdb := redis.NewFailoverClient(&redis.FailoverOptions{
		MasterName:    "master",
		SentinelAddrs: []string{":6379"},
	})
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	r, err := rdb.Ping(ctx).Result()
	if err != nil {
		panic(err)
	}
	println("redis cluster connection success: ", r)
}

func TestExample01_02(t *testing.T) {
	rdb := redis.NewClusterClient(&redis.ClusterOptions{
		Addrs: []string{":6379"},
	})
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	r, err := rdb.Ping(ctx).Result()
	if err != nil {
		panic(err)
	}
	println("redis cluster connection success: ", r)
}
