// Under the MIT License.  See LICENSE file for details.

package display_test

import (
	"bytes"
	"testing"

	"github.com/groboclown/native-shell/lib/display"
)

type println struct {
	in       string
	expected string
}

var printlnTests = []println{
	{"", "\n"},
	{"a", "a\n"},
	{"a b c", "a b c\n"},
}

func Test_Println(t *testing.T) {
	for _, p := range printlnTests {
		var out, e bytes.Buffer
		d := display.NewDisplay(10, &out, &e)
		if err := d.Println(p.in); err != nil {
			t.Errorf("Println encountered an error: %v", err)
		}
		if o := out.String(); o != p.expected {
			t.Errorf("Println generated '%s', expected '%s'", o, p.expected)
		}
		if o := e.String(); o != "" {
			t.Errorf("Println generated err '%s'", o)
		}
	}
}

func Test_PrintlnBytes(t *testing.T) {
	for _, p := range printlnTests {
		var out, e bytes.Buffer
		d := display.NewDisplay(10, &out, &e)
		if err := d.PrintlnBytes([]byte(p.in)); err != nil {
			t.Errorf("PrintlnBytes encountered an error: %v", err)
		}
		if o := out.String(); o != p.expected {
			t.Errorf("PrintlnBytes generated '%s', expected '%s'", o, p.expected)
		}
		if o := e.String(); o != "" {
			t.Errorf("PrintlnBytes generated err '%s'", o)
		}
	}
}

func Test_Errln(t *testing.T) {
	for _, p := range printlnTests {
		var out, e bytes.Buffer
		d := display.NewDisplay(10, &out, &e)
		if err := d.Errln(p.in); err != nil {
			t.Errorf("Errln encountered an error: %v", err)
		}
		if o := e.String(); o != p.expected {
			t.Errorf("Errln generated '%s', expected '%s'", o, p.expected)
		}
		if o := out.String(); o != "" {
			t.Errorf("Errln generated stdout '%s'", o)
		}
	}
}

func Test_ErrlnBytes(t *testing.T) {
	for _, p := range printlnTests {
		var out, e bytes.Buffer
		d := display.NewDisplay(10, &out, &e)
		if err := d.ErrlnBytes([]byte(p.in)); err != nil {
			t.Errorf("ErrlnBytes encountered an error: %v", err)
		}
		if o := e.String(); o != p.expected {
			t.Errorf("ErrlnBytes generated '%s', expected '%s'", o, p.expected)
		}
		if o := out.String(); o != "" {
			t.Errorf("ErrlnBytes generated stdout '%s'", o)
		}
	}
}
