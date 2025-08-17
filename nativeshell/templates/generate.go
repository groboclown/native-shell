// Under the MIT License.  See LICENSE file for details.

package templates

import (
	"errors"
	"fmt"

	"github.com/groboclown/native-shell/nativeshell/ast"
)

// GenerateCode generates the actual source code from the shell template.
func (s *ShellTemplate) GenerateCode(source ast.Source, named ast.Ref, parameters ast.Parameters) (string, error) {

}

func generateParameterValues(
	context map[string]any,
	source ast.Source,
	named ast.Ref,
	parameterTypes map[string]ParameterType,
	parameterValues ast.Parameters,
) (map[string]any, error) {
	ret := context
	problems := make([]error, 0)

	for key, typed := range parameterTypes {
		if param, ok := parameterValues[key]; ok {
			continue
		}
		problems = append(problems, fmt.Errorf("%s: Required parameter '%s' not given for %s"))
	}

	return ret, errors.Join(problems...)
}
