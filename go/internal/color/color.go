package color

import (
	"fmt"
	"image/color"
	"math/rand"
)

type RGB struct {
	R, G, B float64
}

func New(r, g, b float64) RGB {
	return RGB{R: r, G: g, B: b}
}

func (c RGB) Add(d RGB) RGB {
	return New(c.R+d.R, c.G+d.G, c.B+d.B)
}

func (c RGB) Sub(d RGB) RGB {
	return New(c.R-d.R, c.G-d.G, c.B-d.B)
}

func (c RGB) Mul(d float64) RGB {
	return New(c.R*d, c.G*d, c.B*d)
}

func (c RGB) Div(d float64) RGB {
	return c.Mul(1 / d)
}

func (c RGB) Attenuate(d RGB) RGB {
	return New(c.R*d.R, c.G*d.G, c.B*d.B)
}

// TODO: Does this constant make sense?
const encodeScale = 255.999

func (c RGB) Encode() color.RGBA {
	return color.RGBA{
		R: uint8(c.R * encodeScale),
		G: uint8(c.G * encodeScale),
		B: uint8(c.B * encodeScale),
		A: 255,
	}
}

func (c RGB) String() string {
	return fmt.Sprintf("(%.1f, %.1f, %.1f)", c.R*encodeScale, c.G*encodeScale, c.B*encodeScale)
}

func Random(random *rand.Rand) RGB {
	return New(random.Float64(), random.Float64(), random.Float64())
}

func RandomRange(random *rand.Rand, min, max float64) RGB {
	return New(min, min, min).Add(Random(random).Mul(max - min))
}
