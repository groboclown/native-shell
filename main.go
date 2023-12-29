// Under the MIT License.  See LICENSE file for details.

package nativeshell

import (
	"os"

	"github.com/groboclown/native-shell/lib/cmd"
	"github.com/groboclown/native-shell/lib/display"
)

func main() {
	os.Exit(cmd.ParseArgs(os.Args[0], os.Args[1:]).Run(display.New()))
}
