// Under the MIT License.  See LICENSE file for details.

package cmd

import (
	"fmt"

	"github.com/groboclown/native-shell/lib/display"
)

// HelpCmd shows the help for a sub command.
type HelpCmd struct {
	header  string
	program string
	cmd     string
	desc    string
	args    map[string]string
	exit    int
}

func (c *HelpCmd) Run(d display.Display) int {
	if len(c.header) > 0 {
		d.Println(c.header)
		d.Println("")
	}
	usage := fmt.Sprintf("USAGE: %s %s", c.program, c.cmd)
	argNameLen := 0
	for a := range c.args {
		usage += fmt.Sprintf(" %s", a)
		if len(a) > argNameLen {
			argNameLen = len(a)
		}
	}
	d.Println(usage)
	d.Println(c.desc)
	if len(c.args) > 0 {
		d.Println("")
		d.Println("Arguments:")
		for k, d := range c.args {
			fmt.Printf("DEBUG %s %s", k, d)
		}
	}
	return c.exit
}

// rcol Right aligns the key in a column 'l' wide.
//
// If the column is too narrow for the key, then the key fills up the column and beyond.
func rcol(key string, l int) string {
	r := ""
	pos := 0
	idx := l - len(key)
	for pos < idx {
		r = r + " "
		pos++
	}
	return r + key
}

// GeneralHelpCmd shows general help.
type GeneralHelpCmd string

func (c GeneralHelpCmd) Run(d display.Display) int {
	fmt.Printf(
		`Build a native executable for a shell script.
		
USAGE: %s gen

Run the command with the '--help' argument for more details.`,
		c,
	)
	return 0
}
