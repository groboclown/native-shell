// Under the MIT License.  See LICENSE file for details.

package problems

import (
	"fmt"

	"github.com/groboclown/native-shell/nativeshell/ast"
)

type ScriptError struct {
	Source  ast.Source
	Ref     ast.Ref
	Message string
}

func NewScriptErrorRef(
	source ast.Source,
	ref ast.Ref,
	msg string,
	params ...any,
) *ScriptError {
	return &ScriptError{
		Source:  source,
		Ref:     ref,
		Message: fmt.Sprintf(msg, params...),
	}
}

func NewScriptError(
	source ast.Source,
	msg string,
	params ...any,
) *ScriptError {
	return NewScriptErrorRef(source, "", msg, params)
}

func (e *ScriptError) Error() string {
	if len(e.Ref) > 0 {
		return fmt.Sprintf("")
	}
}
