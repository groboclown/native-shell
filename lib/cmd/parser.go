// Under the MIT License.  See LICENSE file for details.

package cmd

import (
	"strings"

	"github.com/groboclown/native-shell/lib/display"
)

// Command represents a runnable command.
type Command interface {
	Run(display.Display) int
}

func ParseArgs(programName string, args []string) Command {
	namedValues := make(map[string]string)
	flags := make(map[string]bool, 0)
	nonFlags := make([]string, 0)
	state := stateSeek

	for _, a := range args {
		switch state {

		case stateRaw:
			nonFlags = append(nonFlags, a)

		case stateSeek:
			switch {
			case a == "--":
				state = stateRaw
			case len(a) > 2 && a[0:1] == "--":
				if k, v, hasVal := ParseLongArg(a[2:]); hasVal {
					namedValues[k] = v
				} else {
					flags[k] = true
				}
			default:
				nonFlags = append(nonFlags, a)
			}
		}
	}

	if len(nonFlags) <= 0 {
		return GeneralHelpCmd(programName)
	}
	panic("Not finished")
}

const stateSeek = 0
const stateRaw = 1

// ParseLongArg parses a single argument in the form 'foo' or 'foo=bar'.
func ParseLongArg(val string) (key string, value string, hasValue bool) {
	pos := strings.IndexByte(val, '=')
	if pos <= 0 {
		// Note the special case: a single '=' character.  It's returned as the unadorned key.
		return val, "", false
	}
	return val[:pos], val[pos+1:], true
}
