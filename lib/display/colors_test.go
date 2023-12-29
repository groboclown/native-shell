// Under the MIT License.  See LICENSE file for details.

package display_test

import (
	"testing"

	"github.com/TwiN/go-color"
	"github.com/google/go-cmp/cmp"
	"github.com/groboclown/native-shell/lib/display"
)

type blockConv struct {
	b display.TextBlock
	p string
	c string
}

var blockConvTests = []blockConv{
	{
		display.TextBlock{},
		"",
		"",
	},
	{
		display.TextBlock{{}, {}},
		"",
		"",
	},
	{
		display.TextBlock{
			{T: "a"},
		},
		"a",
		"a",
	},
	{
		display.TextBlock{
			{T: "a", X: display.Bold | display.Underline},
			{T: "b", X: display.Bold},
			{T: "c"},
		},
		"abc",
		color.Underline + color.Bold + "a" + color.Reset + color.Bold + "b" + color.Reset + "c",
	},
}

func Test_ConvertBytes(t *testing.T) {
	for _, test := range blockConvTests {
		bc1, err := test.b.Bytes(true)
		if err != nil {
			t.Errorf("Bytes(true) of %v generated error %e", test.b, err)
		}
		if d := cmp.Diff([]byte(test.c), bc1); d != "" {
			t.Errorf("Bytes(true) of %v: %s", test.b, d)
		}

		bc2, err := test.b.ColorBytes()
		if err != nil {
			t.Errorf("ColorBytes() of %v generated error %e", test.b, err)
		}
		if d := cmp.Diff([]byte(test.c), bc2); d != "" {
			t.Errorf("ColorBytes() of %v: %s (%s)", test.b, d, test.c)
		}

		bt1, err := test.b.Bytes(false)
		if err != nil {
			t.Errorf("Bytes(false) of %v generated error %e", test.b, err)
		}
		if d := cmp.Diff([]byte(test.p), bt1); d != "" {
			t.Errorf("Bytes(false) of %v: %s", test.b, d)
		}

		bt2, err := test.b.PlainBytes()
		if err != nil {
			t.Errorf("PlainBytes() of %v generated error %e", test.b, err)
		}
		if d := cmp.Diff([]byte(test.p), bt2); d != "" {
			t.Errorf("PlainBytes() of %v: %s", test.b, d)
		}
	}
}
