package main

import (
	"fmt"
	"io"
	"net/http"
)

func search(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	search := query.Get("s");
	fmt.Printf("%s", search);

	io.WriteString(w, "This is my website!\n")
}

func getHello(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("got /hello request\n")
	io.WriteString(w, "Hello, HTTP!\n")
}

func main() {
	http.HandleFunc("/search", search)
	http.HandleFunc("/hello", getHello)

	_ = http.ListenAndServe(":3333", nil)
}