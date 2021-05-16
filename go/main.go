package main

import (
	"flag"
	"fmt"
	"image"
	"image/png"
	"math/rand"
	"os"
	"runtime"

	"github.com/nya3jp/raytracing/go/internal/renderer"
	"github.com/nya3jp/raytracing/go/internal/scene"
)

func savePNG(im *image.RGBA, filename string) (retErr error) {
	f, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer func() {
		if err := f.Close(); err != nil && retErr == nil {
			retErr = err
		}
	}()
	if err := png.Encode(f, im); err != nil {
		return err
	}
	return nil
}

func main() {
	var outFile, sceneName string
	var width, height, threads int
	var seed int64
	flag.StringVar(&outFile, "out", "out.png", "output filename")
	flag.StringVar(&sceneName, "scene", "spheres", "scene name")
	flag.IntVar(&width, "width", 400, "image width")
	flag.IntVar(&height, "height", 225, "image height")
	flag.IntVar(&threads, "threads", runtime.NumCPU(), "number of threads")
	flag.Int64Var(&seed, "seed", 283, "random seed")
	flag.Parse()

	aspectRatio := float64(width) / float64(height)
	camera, objects, ok := scene.ByName(sceneName, aspectRatio, rand.New(rand.NewSource(seed)))
	if !ok {
		panic(fmt.Sprintf("Unknown scene: %s", sceneName))
	}

	im := renderer.Render(camera, objects, width, height, threads, seed)

	if err := savePNG(im, outFile); err != nil {
		panic(err)
	}
}
