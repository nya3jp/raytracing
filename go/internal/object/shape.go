package object

import (
	"fmt"
	"math"

	"github.com/nya3jp/raytracing/go/internal/geom"
)

type Hit struct {
	Point, Normal geom.Vec
	T             float64
}

func (h Hit) String() string {
	return fmt.Sprintf("Hit(Point=%v, Normal=%v, T=%.3f)", h.Point, h.Normal, h.T)
}

type Shape interface {
	Hit(ray *geom.Ray, tMin, tMax float64) (Hit, bool)
}

type Sphere struct {
	Center geom.Vec
	Radius float64
}

func NewSphere(center geom.Vec, radius float64) *Sphere {
	return &Sphere{Center: center, Radius: radius}
}

func (s *Sphere) Hit(ray *geom.Ray, tMin, tMax float64) (Hit, bool) {
	oc := ray.Origin.Sub(s.Center)
	a := ray.Dir.Norm()
	b2 := ray.Dir.Dot(oc)
	c := oc.Norm() - s.Radius*s.Radius
	discriminant := b2*b2 - a*c
	if discriminant < 0 {
		return Hit{}, false
	}

	droot := math.Sqrt(discriminant)
	t := (-b2 - droot) / a
	if t < tMin || tMax < t {
		t = (-b2 + droot) / a
		if t < tMin || tMax < t {
			return Hit{}, false
		}
	}

	p := ray.At(t)
	n := p.Sub(s.Center).Div(s.Radius)
	return Hit{Point: p, Normal: n, T: t}, true
}

func (s *Sphere) String() string {
	return fmt.Sprintf("Sphere(center=%v, radius=%.3f)", s.Center, s.Radius)
}
