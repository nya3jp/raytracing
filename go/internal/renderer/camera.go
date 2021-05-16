package renderer

import (
	"math"
	"math/rand"

	"github.com/nya3jp/raytracing/go/internal/geom"
)

type Camera struct {
	origin          geom.Vec
	lowerLeftCorner geom.Vec
	horizontal      geom.Vec
	vertical        geom.Vec
	u               geom.Vec
	v               geom.Vec
	lensRadius      float64
}

func NewCamera(lookFrom, lookAt, up geom.Vec, fov, aspectRatio, aperture, focusDist float64) *Camera {
	viewportHeight := 2 * math.Tan(fov/2)
	viewportWidth := viewportHeight * aspectRatio

	w := lookAt.Sub(lookFrom).Unit()
	u := w.Cross(up).Unit()
	v := u.Cross(w)

	horizontal := u.Mul(focusDist * viewportWidth)
	vertical := v.Mul(focusDist * viewportHeight)
	lowerLeftCorner := lookFrom.Add(w.Mul(focusDist)).Sub(horizontal.Div(2)).Sub(vertical.Div(2))
	return &Camera{
		origin:          lookFrom,
		lowerLeftCorner: lowerLeftCorner,
		horizontal:      horizontal,
		vertical:        vertical,
		u:               u,
		v:               v,
		lensRadius:      aperture / 2,
	}
}

func (c *Camera) Ray(s, t float64, random *rand.Rand) *geom.Ray {
	lens := geom.RandomVecInUnitDisc(random).Mul(c.lensRadius)
	blur := c.u.Mul(lens.X).Add(c.v.Mul(lens.Y))
	origin := c.origin.Add(blur)
	dir := c.lowerLeftCorner.Add(c.horizontal.Mul(s)).Add(c.vertical.Mul(t)).Sub(origin)
	return geom.NewRay(origin, dir)
}
