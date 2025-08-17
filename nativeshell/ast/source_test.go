// Under the MIT License.  See LICENSE file for details.

package ast_test

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/groboclown/native-shell/nativeshell/ast"
)

func Test_Localize(t *testing.T) {
	if diff := cmp.Diff(
		(&ast.Source{
			File:  "a.txt",
			Start: ast.SourceFilePosition{Line: 2, Column: 6},
			End:   ast.SourceFilePosition{Line: 3, Column: 7},
		}).Localize(),
		"a.txt Line 2, Column 6",
	); diff != "" {
		t.Errorf(diff)
	}
}
