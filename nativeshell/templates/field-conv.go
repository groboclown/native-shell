// Under the MIT License.  See LICENSE file for details.

package templates

import "fmt"

// ParseFieldType parses the name into a field type.
func ParseFieldType(name string) (FieldType, error) {
	if val, ok := fieldTypeNames[name]; ok {
		return val, nil
	}
	return FieldType{}, fmt.Errorf("unknown field type %s", name)
}

var fieldTypeNames map[string]FieldType = map[string]FieldType{
	"[]string": {IsArray: true, Type: FieldTypeString},
	"string":   {IsArray: false, Type: FieldTypeString},
	"[]bool":   {IsArray: true, Type: FieldTypeBool},
	"bool":     {IsArray: false, Type: FieldTypeBool},
	"[]int":    {IsArray: true, Type: FieldTypeInt},
	"int":      {IsArray: false, Type: FieldTypeInt},
	"[]float":  {IsArray: true, Type: FieldTypeFloat},
	"float":    {IsArray: false, Type: FieldTypeFloat},
}
