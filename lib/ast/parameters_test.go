// Under the MIT License.  See LICENSE file for details.

package ast_test

import (
	"testing"

	"github.com/groboclown/native-shell/lib/ast"
)

func Test_IntParamterValue_str(t *testing.T) {
	v, ok := ast.IntParamterValue("a")
	if ok {
		t.Errorf("ok is not false, but %v", ok)
	}
	if v != 0 {
		t.Errorf("value is not zero, but %v", v)
	}
}

func Test_IntParamterValue_int(t *testing.T) {
	v, ok := ast.IntParamterValue(12)
	if ok != true {
		t.Errorf("ok is not true, but %v", ok)
	}
	if v != 12 {
		t.Errorf("value is not 12, but %v", v)
	}
}

func Test_IntParamterValue_int16(t *testing.T) {
	v, ok := ast.IntParamterValue(int16(12))
	if ok != true {
		t.Errorf("ok is not true, but %v", ok)
	}
	if v != 12 {
		t.Errorf("value is not 12, but %v", v)
	}
}

func Test_IntParamterValue_int32(t *testing.T) {
	v, ok := ast.IntParamterValue(int32(12))
	if ok != true {
		t.Errorf("ok is not true, but %v", ok)
	}
	if v != 12 {
		t.Errorf("value is not 12, but %v", v)
	}
}

func Test_IntParamterValue_int64(t *testing.T) {
	v, ok := ast.IntParamterValue(int64(12))
	if ok != true {
		t.Errorf("ok is not true, but %v", ok)
	}
	if v != 12 {
		t.Errorf("value is not 12, but %v", v)
	}
}

func Test_IntParamterValue_float64(t *testing.T) {
	v, ok := ast.IntParamterValue(float64(12))
	if ok {
		t.Errorf("ok is not false, but %v", ok)
	}
	if v != 0 {
		t.Errorf("value is not zero, but %v", v)
	}
}

func Test_ParseBoolParameterValue(t *testing.T) {
	if v, ok := ast.ParseBoolParameterValue(false); v != false || ok != true {
		t.Errorf("false should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(true); v != true || ok != true {
		t.Errorf("true should be true, true, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(int(0)); v != false || ok != true {
		t.Errorf("int(0) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int(1)); v != true || ok != true {
		t.Errorf("int(1) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int(3)); v != false || ok != false {
		t.Errorf("int(3) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(int16(0)); v != false || ok != true {
		t.Errorf("int16(0) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int16(1)); v != true || ok != true {
		t.Errorf("int16(1) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int16(3)); v != false || ok != false {
		t.Errorf("int16(3) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(int32(0)); v != false || ok != true {
		t.Errorf("int32(0) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int32(1)); v != true || ok != true {
		t.Errorf("int32(1) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int32(3)); v != false || ok != false {
		t.Errorf("int32(3) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(int64(0)); v != false || ok != true {
		t.Errorf("int64(0) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int64(1)); v != true || ok != true {
		t.Errorf("int64(1) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(int64(3)); v != false || ok != false {
		t.Errorf("int64(3) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(float32(-0.01)); v != false || ok != true {
		t.Errorf("float32(-0.01) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float32(0.01)); v != false || ok != true {
		t.Errorf("float32(0.01) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float32(0.901)); v != true || ok != true {
		t.Errorf("float32(0.901) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float32(1.01)); v != true || ok != true {
		t.Errorf("float32(1.01) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float32(-1.01)); v != false || ok != false {
		t.Errorf("float32(-1.01) should be false, false, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float32(2.01)); v != false || ok != false {
		t.Errorf("float32(2.01) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue(float64(-0.01)); v != false || ok != true {
		t.Errorf("float64(-0.01) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float64(0.01)); v != false || ok != true {
		t.Errorf("float64(0.01) should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float64(0.901)); v != true || ok != true {
		t.Errorf("float64(0.901) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float64(1.01)); v != true || ok != true {
		t.Errorf("float64(1.01) should be true, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float64(-1.01)); v != false || ok != false {
		t.Errorf("float64(-1.01) should be false, false, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue(float64(2.01)); v != false || ok != false {
		t.Errorf("float64(2.01) should be false, false, but found %v, %v", v, ok)
	}

	if v, ok := ast.ParseBoolParameterValue("False"); v != false || ok != true {
		t.Errorf("false should be false, true, but found %v, %v", v, ok)
	}
	if v, ok := ast.ParseBoolParameterValue("TRUE"); v != true || ok != true {
		t.Errorf("true should be true, true, but found %v, %v", v, ok)
	}

}
