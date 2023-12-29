// Under the MIT License.  See LICENSE file for details.

package cmd

import (
	"fmt"

	"github.com/groboclown/native-shell/lib/display"
)

// InvalidCmd represents a bad parsing of the arguments.
type InvalidCmd string

// Run reports the problems running the command.
func (c InvalidCmd) Run(display.Display) int {
	fmt.Printf("Invalid invocation: %s\n", c)
	return 1
}
