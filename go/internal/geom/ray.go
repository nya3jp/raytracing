package geom

import (
	"fmt"
)

type Ray struct {
	Origin, Dir Vec
}

func NewRay(origin, dir Vec) *Ray {
	return &Ray{Origin: origin, Dir: dir}
}

func (r *Ray) At(t float64) Vec {
	return r.Origin.Add(r.Dir.Mul(t))
}

func (r *Ray) String() string {
	return fmt.Sprintf("Ray(Origin=%v, Dir=%v)", r.Origin, r.Dir)
}
