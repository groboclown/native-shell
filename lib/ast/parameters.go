// Under the MIT License.  See LICENSE file for details.

package ast

import (
	"fmt"
	"strings"
)

// IntParamterValue converts the param value to an int64, only if it is exactly an int64.
func IntParamterValue(param ParameterValue) (value int64, ok bool) {
	switch v := param.(type) {
	case int:
		return int64(v), true
	case int16:
		return int64(v), true
	case int32:
		return int64(v), true
	case int64:
		return v, true
	default:
		return 0, false
	}
}

// FloatParameterValue converts the param value to a float64, only if it is exactly a float64.
func FloatParameterValue(param ParameterValue) (value float64, ok bool) {
	switch v := param.(type) {
	case int:
		return float64(v), true
	case int16:
		return float64(v), true
	case int32:
		return float64(v), true
	case int64:
		return float64(v), true
	case float32:
		return float64(v), true
	case float64:
		return v, true
	default:
		return 0.0, false
	}
}

// BoolParameterValue converts the param value to a boolean, only if it is exactly a boolean.
func BoolParameterValue(param ParameterValue) (value bool, ok bool) {
	v, ok := param.(bool)
	return v, ok
}

// StringParameterValue converts the param value to a string, only if it is exactly a string.
func StringParameterValue(param ParameterValue) (value string, ok bool) {
	v, ok := param.(string)
	return v, ok
}

// ParseBoolParameterValue converts the param value to a boolean, only if it is exactly a boolean.
func ParseBoolParameterValue(param ParameterValue) (value bool, ok bool) {
	switch v := param.(type) {
	case bool:
		return v, true
	case int:
		return intAsBool(int64(v))
	case int16:
		return intAsBool(int64(v))
	case int32:
		return intAsBool(int64(v))
	case int64:
		return intAsBool(v)
	case float32:
		return floatAsBool(float64(v))
	case float64:
		return floatAsBool(v)
	case string:
		return stringAsBool(v)
	default:
		return false, false
	}
}

func intAsBool(v int64) (bool, bool) {
	switch v {
	case 0:
		return false, true
	case 1:
		return true, true
	default:
		return false, false
	}
}

func floatAsBool(v float64) (bool, bool) {
	if v >= -0.1 && v <= 0.1 {
		return false, true
	}
	if v >= 0.9 && v <= 1.1 {
		return true, true
	}
	return false, false
}

func stringAsBool(v string) (bool, bool) {
	b, ok := boolNames[strings.ToLower(v)]
	return b, ok
}

var boolNames = map[string]bool{
	"true":        true,
	"false":       false,
	"t":           true,
	"f":           false,
	"on":          true,
	"off":         false,
	"active":      true,
	"activated":   true,
	"deactivated": false,
	"enabled":     true,
	"disabled":    false,
	"0":           false,
	"1":           true,
}

// ParseStringParameterValue converts the param value to a string.
func ParseStringParameterValue(param ParameterValue) string {
	switch v := param.(type) {
	case int:
		return fmt.Sprintf("%d", v)
	case int16:
		return fmt.Sprintf("%d", v)
	case int32:
		return fmt.Sprintf("%d", v)
	case int64:
		return fmt.Sprintf("%d", v)
	case float32:
		return fmt.Sprintf("%f", v)
	case float64:
		return fmt.Sprintf("%f", v)
	case string:
		return v
	default:
		return fmt.Sprintf("%v", v)
	}
}
