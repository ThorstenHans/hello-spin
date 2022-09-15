package main

import (
	"fmt"

	"github.com/fermyon/spin/sdk/go/redis"
)

func init() {
	// redis.Handle() must be called in the init() function.
	redis.Handle(func(payload []byte) error {
		fmt.Println(string(payload))
		return nil
	})
}

func main() {}
