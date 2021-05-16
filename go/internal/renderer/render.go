package renderer

import (
	"image"
	"log"
	"math"
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/color"
	"github.com/nya3jp/raytracing/go/internal/geom"
	"github.com/nya3jp/raytracing/go/internal/object"
)

var Debug = false

const infinity = 1e100

func hitObject(objs []*object.Object, ray *geom.Ray, tMin, tMax float64) (*object.Object, object.Hit, bool) {
	var nearestObj *object.Object
	var nearestHit object.Hit
	anyHit := false
	for _, obj := range objs {
		if hit, ok := obj.Shape.Hit(ray, tMin, tMax); ok {
			if !anyHit || hit.T < nearestHit.T {
				nearestObj = obj
				nearestHit = hit
				anyHit = true
			}
		}
	}
	return nearestObj, nearestHit, anyHit
}

const reflectionLimit = 50

func renderSky(ray *geom.Ray) color.RGB {
	t := 0.5 * (ray.Dir.Unit().Y + 1) // [0, 1]
	return color.New(1, 1, 1).Mul(1 - t).Add(color.New(0.5, 0.7, 1).Mul(t))
}

func RenderRay(ray *geom.Ray, objects []*object.Object, random *rand.Rand, depth int) color.RGB {
	if Debug {
		log.Printf("Ray: %v", ray)
	}
	if depth >= reflectionLimit {
		if Debug {
			log.Printf("End: Dark")
		}
		return color.New(0, 0, 0)
	}
	if obj, hit, ok := hitObject(objects, ray, 1e-3, infinity); ok {
		color, reflection, reflected := obj.Material.Scatter(ray, hit, random)
		if !reflected {
			if Debug {
				log.Printf("End: Opaque %v", color)
			}
			return color
		}
		if Debug {
			log.Printf("Reflect: Attenuate %v", color)
		}
		return RenderRay(reflection, objects, random, depth+1).Attenuate(color)
	}
	if Debug {
		log.Printf("Reflect: Sky")
	}
	return renderSky(ray)
}

func Render(camera *Camera, objects []*object.Object, width, height, threads int, seed int64) *image.RGBA {
	const samples = 100
	im := image.NewRGBA(image.Rect(0, 0, width, height))

	queue := newQueue(threads)
	for i := 0; i < height; i++ {
		i := i
		queue.Add(func() {
			random := rand.New(rand.NewSource(seed))
			for j := 0; j < width; j++ {
				cl := color.New(0, 0, 0)
				for s := 0; s < samples; s++ {
					u := (float64(j) + random.Float64()) / float64(width)
					v := (float64(height-1) - float64(i) + random.Float64()) / float64(height)
					ray := camera.Ray(u, v, random)
					cl = cl.Add(RenderRay(ray, objects, random, 0))
				}
				cl = cl.Div(float64(samples))
				cl = color.New(math.Sqrt(cl.R), math.Sqrt(cl.G), math.Sqrt(cl.B))
				im.SetRGBA(j, i, cl.Encode())
			}
		})
	}
	queue.Run()
	return im
}
