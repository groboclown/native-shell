// Under the MIT License.  See LICENSE file for details.

package display

import (
	"fmt"
	"io"
	"strings"
)

// PrintlnBytes sends the bytes, as a full line, to the display out.
//
// If the byte array does not end with a newline character, one is added.
func (d *Display) PrintlnBytes(t []byte) error {
	return outPrintlnBytes(d.out, t)
}

func (d *Display) Text(tb TextBlock) error {
	return outPrintlnText(d.out, tb, d.Colorize)
}

func (d *Display) Println(t string) error {
	return outPrintln(d.out, t)
}

func (d *Display) Printlnf(t string, a ...any) error {
	return outPrintln(d.out, fmt.Sprintf(t, a...))
}

func (d *Display) PrintlnParts(t []string) error {
	return outPrintlnParts(d.out, t)
}

func (d *Display) PrintlnPartsJoin(t []string, joiner string) error {
	return outPrintlnPartsJoin(d.out, t, joiner)
}

// PrintlnBytes sends the bytes, as a full line, to the display err.
//
// If the byte array does not end with a newline character, one is added.
func (d *Display) ErrlnBytes(t []byte) error {
	return outPrintlnBytes(d.err, t)
}

func (d *Display) ErrText(tb TextBlock) error {
	return outPrintlnText(d.err, tb, d.Colorize)
}

func (d *Display) Errln(t string) error {
	return outPrintln(d.err, t)
}

func (d *Display) Errlnf(t string, a ...any) error {
	return outPrintln(d.err, fmt.Sprintf(t, a...))
}

func (d *Display) ErrlnParts(t []string) error {
	return outPrintlnParts(d.err, t)
}

func (d *Display) ErrlnPartsJoin(t []string, joiner string) error {
	return outPrintlnPartsJoin(d.err, t, joiner)
}

func outPrintlnText(out io.Writer, tb TextBlock, colorize bool) error {
	b, err := tb.Bytes(colorize)
	if err != nil {
		return err
	}
	return outPrintlnBytes(out, b)
}

func outPrintlnBytes(out io.Writer, t []byte) error {
	r := t
	n := len(t)
	if n == 0 || r[n-1] != '\n' {
		r = make([]byte, 0)
		r = append(r, t...)
		r = append(r, '\n')
	}
	_, err := out.Write([]byte(r))
	return err
}

func outPrintln(out io.Writer, t string) error {
	return outPrintlnBytes(out, []byte(t))
}

func outPrintlnParts(out io.Writer, t []string) error {
	return outPrintlnPartsJoin(out, t, "")
}

func outPrintlnPartsJoin(out io.Writer, t []string, joiner string) error {
	return outPrintln(out, strings.Join(t, joiner))
}
