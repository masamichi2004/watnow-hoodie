package main

import (
    "fmt"
    x "strings"
    sx "slices"
)

func d(s string) string {
    var r x.Builder
    for _, c := range s {
        if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
            b := 'A'
            if c >= 'a' {
                b = 'a'
            }
            dd := (c - b + 13) % 26 + b
            r.WriteRune(dd);
        } else {
            r.WriteRune(c);
        };
    }
    return x.ReplaceAll(r.String(), "_", " ");
}

func bd(chs []chan string) string {
    var b x.Builder;
    for i :=0; i<len(chs); i++ {
                for _, ch := range chs {
                    c, ok := <-ch;
                    if ok {
                        b.WriteString(c);
                    }
                }
            };
            r := []rune(d(b.String()));
            sx.Reverse(r);
            return string(r);
        }

func main() {
    ss := [][]string{
        {"!", "a", "e", "K", "r", "r"},
        {"!", "e", "r", "e", "u", "z"},
        {"4", "r", "z", "r", "g", "b"},
        {"2", "g", "z", "i", "_", "p"},
        {"0", "a", "h", "n", "b", "y"},
        {"2", "V", "F", "Y", "g", "r"},
        {"_", "_", "_", "_", "_", "J"},
    };
    chs := make([]chan string, len(ss));
    for i := range chs {
        chs[i] = make(chan string, len(ss[i]));
    };
    for i, sl := range ss {
        go func(ch chan string, s []string) {
            for _, c := range s {
                ch <- c;
            }
            close(ch);
        }( chs[i], sl);
    }
    fmt.Println(bd(chs));
}
