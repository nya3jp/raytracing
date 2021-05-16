package scene

import (
	"math"
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/color"
	"github.com/nya3jp/raytracing/go/internal/geom"
	"github.com/nya3jp/raytracing/go/internal/object"
	"github.com/nya3jp/raytracing/go/internal/renderer"
)

func Sample(aspectRatio float64, _ *rand.Rand) (*renderer.Camera, []*object.Object) {
	camera := renderer.NewCamera(
		geom.NewVec(0, 0, 0),
		geom.NewVec(0, 0, -1),
		geom.NewVec(0, 1, 0),
		math.Pi/2,
		aspectRatio,
		0.1,
		1)
	objects := []*object.Object{
		object.NewObject(object.NewSphere(geom.NewVec(0, -100.5, -1), 100), object.NewLambertian(color.New(0.8, 0.8, 0.0))),
		object.NewObject(object.NewSphere(geom.NewVec(0, 0, -1), 0.5), object.NewLambertian(color.New(0.1, 0.2, 0.5))),
		object.NewObject(object.NewSphere(geom.NewVec(-1, 0, -1), 0.5), object.NewDielectric(1.5)),
		object.NewObject(object.NewSphere(geom.NewVec(-1, 0, -1), -0.45), object.NewDielectric(1.5)),
		object.NewObject(object.NewSphere(geom.NewVec(1, 0, -1), 0.5), object.NewMetal(color.New(0.8, 0.6, 0.2), 0.0)),
	}
	return camera, objects
}
