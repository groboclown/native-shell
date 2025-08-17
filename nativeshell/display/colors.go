// Under the MIT License.  See LICENSE file for details.

package display

import (
	"bytes"

	"github.com/TwiN/go-color"
)

type Color int16
type Special int16

const (
	Unset Color = iota
	Black
	Red
	Green
	Yellow
	Blue
	Purple
	Cyan
	Gray
	White
)

var foregroundMap = map[Color][]byte{
	Black:  []byte(color.Black),
	Red:    []byte(color.Red),
	Green:  []byte(color.Green),
	Yellow: []byte(color.Yellow),
	Blue:   []byte(color.Blue),
	Purple: []byte(color.Purple),
	Cyan:   []byte(color.Cyan),
	Gray:   []byte(color.Gray),
	White:  []byte(color.White),
}

var backgroundMap = map[Color][]byte{
	Black:  []byte(color.BlackBackground),
	Red:    []byte(color.RedBackground),
	Green:  []byte(color.GreenBackground),
	Yellow: []byte(color.YellowBackground),
	Blue:   []byte(color.BlueBackground),
	Purple: []byte(color.PurpleBackground),
	Cyan:   []byte(color.CyanBackground),
	Gray:   []byte(color.GrayBackground),
	White:  []byte(color.WhiteBackground),
}

const Underline Special = 0x1
const Bold Special = 0x2

type Text struct {
	T string
	F Color
	B Color
	X Special
}

type TextBlock []Text

var emptyBytes = []byte{}
var resetBytes = []byte(color.Reset)
var underlineBytes = []byte(color.Underline)
var boldBytes = []byte(color.Bold)

// Bytes extracts the text, either with or without color, based on the colorize argument.
func (tb TextBlock) Bytes(colorize bool) ([]byte, error) {
	if colorize {
		return tb.ColorBytes()
	}
	return tb.PlainBytes()
}

func notNone(v []byte) []byte {
	if v == nil {
		return []byte{}
	}
	return v
}

// PlainBytes extracts only the text from the text array into a byte array.
func (tb TextBlock) PlainBytes() ([]byte, error) {
	var ret bytes.Buffer
	for _, t := range tb {
		if _, err := ret.WriteString(t.T); err != nil {
			return ret.Bytes(), err
		}
	}
	return notNone(ret.Bytes()), nil
}

// ColorBytes extracts the colors and text from the text array into a byte array.
func (tb TextBlock) ColorBytes() ([]byte, error) {
	b, e := tb.colorBytes()
	return notNone(b), e
}

func (tb TextBlock) colorBytes() ([]byte, error) {
	fgc := Unset
	fg := emptyBytes
	bgc := Unset
	bg := emptyBytes
	var bold Special = 0
	var underline Special = 0
	isSet := false
	var ret bytes.Buffer

	for _, t := range tb {
		nu := t.X & Underline
		nb := t.X & Bold
		if (underline != 0 && nu == 0) || (bold != 0 && nb == 0) {
			// The only way to turn off underline or bold is to reset.
			if _, err := ret.Write(resetBytes); err != nil {
				return ret.Bytes(), err
			}

			fgc = Unset
			fg = emptyBytes
			bgc = Unset
			bg = emptyBytes
			underline = 0
			bold = 0
			isSet = false
		}

		nfgc := fgc
		nfg := fg
		if t.F != Unset {
			nfgc = t.F
			nfg = foregroundMap[nfgc]
		}
		if nfgc != fgc {
			if _, err := ret.Write(nfg); err != nil {
				return ret.Bytes(), err
			}
			fgc = nfgc
			fg = nfg
			isSet = true
		}

		nbgc := bgc
		nbg := bg
		if t.B != Unset {
			nbgc = t.B
			nbg = backgroundMap[nbgc]
		}
		if nbgc != bgc {
			if _, err := ret.Write(nbg); err != nil {
				return ret.Bytes(), err
			}
			bgc = nbgc
			bg = nbg
			isSet = true
		}

		if nu != 0 && underline == 0 {
			if _, err := ret.Write(underlineBytes); err != nil {
				return ret.Bytes(), err
			}
			underline = nu
			isSet = true
		}
		if nb != 0 && bold == 0 {
			if _, err := ret.Write(boldBytes); err != nil {
				return ret.Bytes(), err
			}
			bold = nb
			isSet = true
		}

		if _, err := ret.WriteString(t.T); err != nil {
			return ret.Bytes(), err
		}
	}

	if isSet {
		if _, err := ret.Write(resetBytes); err != nil {
			return ret.Bytes(), err
		}
	}

	return ret.Bytes(), nil
}
