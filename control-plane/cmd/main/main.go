package main

import (
	"fmt"
	"net/http"
)

func main() {
	// _ = bootstrap.Start()

	fmt.Println("Control plane server listening on :8080")

	http.ListenAndServe(":8080", nil)
}
