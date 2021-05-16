package object

type Object struct {
	Shape    Shape
	Material Material
}

func NewObject(shape Shape, material Material) *Object {
	return &Object{Shape: shape, Material: material}
}
