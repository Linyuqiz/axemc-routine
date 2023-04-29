package common

import (
	"database/sql"
	_ "github.com/go-sql-driver/mysql"
	"testing"
)

func TestExample02_00(t *testing.T) {
	// 第一范式：每个列都不可以再拆分
	// 第二范式：在第一范式的基础上，非主键列完全依赖于主键，而不能是依赖于主键的一部分
	// 第三范式：在第二范式的基础上，非主键列只依赖于主键，不依赖于其他非主键。
	db, err := sql.Open("mysql", "user:password@tcp(127.0.0.1:3306)/dbname")
	if err != nil {
		panic(err)
	}
	// 注意这行代码要写在上面err判断的下面
	defer db.Close()
	// 尝试与数据库建立连接（校验dsn是否正确）
	err = db.Ping()
	if err != nil {
		panic(err)
	}

	db.SetMaxOpenConns(100)
	db.SetMaxIdleConns(20)

	db.Query("", 1)
	db.QueryRow("", 0)
	db.Exec("", "")

	stmt, err := db.Prepare("")
	if err != nil {
		panic(err)
	}
	defer stmt.Close()
	_ = stmt.QueryRow()

	// ACID：原子性（Atomicity，或称不可分割性）、一致性（Consistency）、隔离性（Isolation，又称独立性）、持久性（Durability）
	tx, err := db.Begin()
	_ = tx.Commit()
	_ = tx.Rollback()
}
