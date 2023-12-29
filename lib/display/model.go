// Under the MIT License.  See LICENSE file for details.

package display

import (
	"io"
	"os"

	"golang.org/x/term"
)

// Display allows helper functions for printing to a console.
type Display struct {
	width    int
	out      io.Writer
	err      io.Writer
	Colorize bool
}

// New creates a new display object.
func New() Display {
	return Display{
		width:    colCount(),
		out:      os.Stdout,
		err:      os.Stderr,
		Colorize: true,
	}
}

// NewFixed creates a new display object, with a fixed width.
func NewFixed(width int) Display {
	return Display{
		width:    width,
		out:      os.Stdout,
		err:      os.Stderr,
		Colorize: true,
	}
}

// NewDisplay creates a new display object, with all parameters defined.
func NewDisplay(width int, out, err io.Writer) Display {
	return Display{
		width:    width,
		out:      out,
		err:      err,
		Colorize: true,
	}
}

func colCount() int {
	width, _, err := term.GetSize(0)
	if err != nil {
		return 10000
	}
	return width
}
