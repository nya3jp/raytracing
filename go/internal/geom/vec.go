package geom

import (
	"fmt"
	"math"
	"math/rand"
)

type Vec struct {
	X, Y, Z float64
}

func NewVec(x, y, z float64) Vec {
	return Vec{X: x, Y: y, Z: z}
}

func (v Vec) Add(u Vec) Vec {
	return NewVec(v.X+u.X, v.Y+u.Y, v.Z+u.Z)
}

func (v Vec) Sub(u Vec) Vec {
	return NewVec(v.X-u.X, v.Y-u.Y, v.Z-u.Z)
}

func (v Vec) Mul(d float64) Vec {
	return NewVec(v.X*d, v.Y*d, v.Z*d)
}

func (v Vec) Div(d float64) Vec {
	return v.Mul(1 / d)
}

func (v Vec) Neg() Vec {
	return NewVec(-v.X, -v.Y, -v.Z)
}

func (v Vec) Dot(u Vec) float64 {
	return v.X*u.X + v.Y*u.Y + v.Z*u.Z
}

func (v Vec) Cross(u Vec) Vec {
	return NewVec(v.Y*u.Z-v.Z*u.Y, v.Z*u.X-v.X*u.Z, v.X*u.Y-v.Y*u.X)
}

func (v Vec) Norm() float64 {
	return v.Dot(v)
}

func (v Vec) Abs() float64 {
	return math.Sqrt(v.Norm())
}

func (v Vec) Unit() Vec {
	return v.Div(v.Abs())
}

func (v Vec) String() string {
	return fmt.Sprintf("(%.1f, %.1f, %.1f)", v.X, v.Y, v.Z)
}

func RandomVecInUnitSphere(random *rand.Rand) Vec {
	// TODO: Avoid the infinite loop.
	for {
		x := random.Float64()*2 - 1
		y := random.Float64()*2 - 1
		z := random.Float64()*2 - 1
		v := NewVec(x, y, z)
		if v.Norm() <= 1 {
			return v
		}
	}
}

func RandomVecInUnitDisc(random *rand.Rand) Vec {
	// TODO: Avoid the infinite loop.
	for {
		x := random.Float64()*2 - 1
		y := random.Float64()*2 - 1
		v := NewVec(x, y, 0)
		if v.Norm() <= 1 {
			return v
		}
	}
}
