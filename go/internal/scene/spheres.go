package scene

import (
	"math"
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/color"
	"github.com/nya3jp/raytracing/go/internal/geom"
	"github.com/nya3jp/raytracing/go/internal/object"
	"github.com/nya3jp/raytracing/go/internal/renderer"
)

func Spheres(aspectRatio float64, random *rand.Rand) (*renderer.Camera, []*object.Object) {
	camera := renderer.NewCamera(
		geom.NewVec(13, 2, 3),
		geom.NewVec(0, 0, 0),
		geom.NewVec(0, 1, 0),
		math.Pi/9,
		aspectRatio,
		0.1,
		10)

	ground := object.NewObject(
		object.NewSphere(geom.NewVec(0, -1000, 0), 1000),
		object.NewLambertian(color.New(0.5, 0.5, 0.5)))
	large1 := object.NewObject(
		object.NewSphere(geom.NewVec(0, 1, 0), 1),
		object.NewDielectric(1.5))
	large2 := object.NewObject(
		object.NewSphere(geom.NewVec(-4, 1, 0), 1),
		object.NewLambertian(color.New(0.4, 0.2, 0.1)))
	large3 := object.NewObject(
		object.NewSphere(geom.NewVec(4, 1, 0), 1),
		object.NewMetal(color.New(0.7, 0.6, 0.5), 0))
	var smalls []*object.Object
	for a := -11; a < 11; a++ {
		for b := -11; b < 11; b++ {
			center := geom.NewVec(float64(a)+0.9*random.Float64(), 0.2, float64(b)+0.9*random.Float64())
			if center.Sub(geom.NewVec(4, 0.2, 0)).Abs() < 0.9 {
				continue
			}
			var mat object.Material
			switch r := random.Float64(); {
			case r < 0.8:
				mat = object.NewLambertian(color.Random(random).Attenuate(color.Random(random)))
			case r < 0.95:
				albedo := color.RandomRange(random, 0.5, 1)
				fuzz := random.Float64() * 0.5
				mat = object.NewMetal(albedo, fuzz)
			default:
				mat = object.NewDielectric(1.5)
			}
			smalls = append(smalls, object.NewObject(object.NewSphere(center, 0.2), mat))
		}
	}
	objects := append([]*object.Object{ground, large1, large2, large3}, smalls...)
	return camera, objects
}
