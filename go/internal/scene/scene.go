package scene

import (
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/object"
	"github.com/nya3jp/raytracing/go/internal/renderer"
)

func ByName(name string, aspectRatio float64, random *rand.Rand) (camera *renderer.Camera, objects []*object.Object, ok bool) {
	switch name {
	case "sample":
		camera, objects = Sample(aspectRatio, random)
	case "spheres":
		camera, objects = Spheres(aspectRatio, random)
	default:
		return nil, nil, false
	}
	return camera, objects, true
}
