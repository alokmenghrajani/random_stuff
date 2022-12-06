package main

import "fmt"
import "os"
import "bufio"
import "errors"

func panicIfNotNil(err err) {
  if err != nil {
    panic(err)
  }
}

func main() {
    fmt.Println("hello world")
    file, err := os.Open("day24.txt") // For read access.
    panicIfNotNil(err)

    if err != nil {
	     log.Fatal(err)
     }
     r := bufio.NewReader(file)
     for true {
         t, _, _ := r.ReadLine()
         println("here: {}", t)
         if len(t) == 0 {
             break;
         }
     }

}
