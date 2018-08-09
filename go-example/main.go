package main

/*
#cgo LDFLAGS: -L../target/release -lsvgtopng
void svg_to_png(char *svg_path, char *png_path);
*/
import "C"

func main() {
	C.svg_to_png(C.CString("atom.svg"), C.CString("atom.png"))
}
