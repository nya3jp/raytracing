package renderer_test

import (
	"math/rand"
	"testing"

	"github.com/nya3jp/raytracing/go/internal/geom"
	"github.com/nya3jp/raytracing/go/internal/object"
	"github.com/nya3jp/raytracing/go/internal/renderer"
)

func TestRenderRay(t *testing.T) {
	renderer.Debug = true
	defer func() { renderer.Debug = false }()

	random := rand.New(rand.NewSource(283))
	objects := []*object.Object{
		object.NewObject(object.NewSphere(geom.NewVec(0, 0, -1), 0.5), object.NewDielectric(1.5)),
	}
	ray := geom.NewRay(geom.NewVec(0, 0, 0), geom.NewVec(0, 0, -1))
	color := renderer.RenderRay(ray, objects, random, 0)
	t.Log(color)
}
