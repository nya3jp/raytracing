package object

import (
	"math"
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/color"
	"github.com/nya3jp/raytracing/go/internal/geom"
)

func reflect(inDir, normal geom.Vec) geom.Vec {
	return inDir.Sub(normal.Mul(normal.Dot(inDir) * 2))
}

func refract(inDir, normal geom.Vec, ratio float64) (geom.Vec, bool) {
	if inDir.Dot(normal) > 0 {
		normal = normal.Neg()
	}
	cos := -inDir.Dot(normal)
	if cos > 1 {
		cos = 1
	}
	sin := math.Sqrt(1 - cos*cos)
	if ratio*sin > 1 {
		return geom.Vec{}, false
	}
	outDirPerp := inDir.Add(normal.Mul(cos)).Mul(ratio)
	outDirPara := normal.Mul(-math.Sqrt(math.Abs(1 - outDirPerp.Norm())))
	return outDirPerp.Add(outDirPara), true
}

type Material interface {
	Scatter(ray *geom.Ray, hit Hit, random *rand.Rand) (color color.RGB, reflection *geom.Ray, reflected bool)
}

type Lambertian struct {
	color color.RGB
}

func NewLambertian(color color.RGB) Lambertian {
	return Lambertian{color: color}
}

func (l Lambertian) Scatter(ray *geom.Ray, hit Hit, random *rand.Rand) (color color.RGB, reflection *geom.Ray, reflected bool) {
	// FIXME: Survey the real Lambertian.
	newDir := hit.Normal.Add(geom.RandomVecInUnitSphere(random).Unit())
	return l.color, geom.NewRay(hit.Point, newDir), true
}

type Metal struct {
	color color.RGB
	fuzz  float64
}

func NewMetal(color color.RGB, fuzz float64) Metal {
	return Metal{color: color, fuzz: fuzz}
}

func (m Metal) Scatter(ray *geom.Ray, hit Hit, random *rand.Rand) (color color.RGB, reflection *geom.Ray, reflected bool) {
	newDir := reflect(ray.Dir, hit.Normal).Add(geom.RandomVecInUnitSphere(random).Mul(m.fuzz))
	return m.color, geom.NewRay(hit.Point, newDir), true
}

type Dielectric struct {
	index float64
}

func NewDielectric(index float64) Dielectric {
	return Dielectric{index: index}
}

func (d Dielectric) Scatter(ray *geom.Ray, hit Hit, random *rand.Rand) (cl color.RGB, reflection *geom.Ray, reflected bool) {
	ratio := d.index
	if ray.Dir.Dot(hit.Normal) < 0 {
		ratio = 1 / ratio
	}
	inDir := ray.Dir.Unit()
	outDir, ok := refract(inDir, hit.Normal, ratio)
	if !ok {
		outDir = reflect(inDir, hit.Normal)
	}
	return color.New(1, 1, 1), geom.NewRay(hit.Point, outDir), true
}
