// Under the MIT License.  See LICENSE file for details.

package display_test

import (
	"testing"

	"github.com/groboclown/native-shell/lib/display"
)

func Test_WordSplit_Empty(t *testing.T) {
	d := display.NewFixed(10)
	if v := d.WordSplit("", display.SplitWith{}); v != "" {
		t.Errorf("Expected zero-length arguments to create a zero length string, but found '%s'", v)
	}
	if v := d.WordSplit("", display.SplitWith{FirstIndent: "w", SecondIndent: "x", LongLineSplitter: "y", Eol: "z"}); v != "wz" {
		t.Errorf("Expected zero-length with indent and eol, but found '%s'", v)
	}
}

func Test_WordSplit_Long(t *testing.T) {
	d := display.NewFixed(10)
	if v := d.WordSplit("01234567890123456789", display.SplitWith{Eol: "x"}); v != "0123456789x0123456789x" {
		t.Errorf("Expected '0123456789x0123456789x', but found '%s'", v)
	}
	if v := d.WordSplit("0123456789", display.SplitWith{
		FirstIndent: "ab", SecondIndent: "cd", LongLineSplitter: "ef", Eol: "gh",
	}); v != "ab012345efghcd6789gh" {
		t.Errorf("Expected 'ab012345efghcd6789gh' with indent and eol, but found '%s'", v)
	}
}

func Test_WordSplit_Words(t *testing.T) {
	d := display.NewFixed(10)
	if v := d.WordSplit("012 345", display.SplitWith{Eol: "x"}); v != "012 345x" {
		t.Errorf("Expected '012 345x', but found '%s'", v)
	}
	if v := d.WordSplit("012 345 678 901", display.SplitWith{Eol: "x"}); v != "012 345x678 901x" {
		t.Errorf("Expected '012 345x678 901x', but found '%s'", v)
	}
}
