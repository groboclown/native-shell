// Under the MIT License.  See LICENSE file for details.

package cmd

import (
	"fmt"

	"github.com/groboclown/native-shell/lib/display"
)

// GenerateSourceCmd contains information for generating source.
type GenerateSourceCmd struct {
	source  string
	libDirs []string
	outDir  string
}

const GenerateSourceName = "gen"

func ParseGenerateSourceArgs(flags map[string]string, args []string) Command {
	if len(args) <= 0 {
		return InvalidCmd("Generate source requires a source file.")
	}
	ret := GenerateSourceCmd{
		source: args[0],
	}
	return &ret
}

// Run creates the source files to compile into a native program.
func (c *GenerateSourceCmd) Run(d display.Display) int {
	fmt.Println("Generate source: not implemented.")
	return 1
}
