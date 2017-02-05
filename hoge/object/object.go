package object

type objectType string

type Object interface {
	Type() objectType
	Inspect() string
}
