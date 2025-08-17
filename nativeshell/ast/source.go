// Under the MIT License.  See LICENSE file for details.

package ast

import "fmt"

// String generates a selection area description of the source in abbreviated format.
func (s *Source) String() string {
	text := ""
	if len(s.File) > 0 {
		text = s.File
	}
	if s.Start.Line > 0 {
		if s.Start.Column > 0 {
			text = fmt.Sprintf("%s@%d.%d", text, s.Start.Line, s.Start.Column)
		} else {
			text = fmt.Sprintf("%s@%d", text, s.Start.Line)
		}
	}
	if s.End.Line > 0 {
		if s.End.Column > 0 {
			text = fmt.Sprintf("%s->%d.%d", text, s.End.Line, s.End.Column)
		} else {
			text = fmt.Sprintf("%s->%d", text, s.End.Line)
		}
	}
	return text
}

// Localize creates a localizable version of the source reference.
func (s *Source) Localize() string {
	// TODO could localize this.
	if len(s.File) > 0 {
		if s.Start.Line > 0 {
			if s.Start.Column > 0 {
				return fmt.Sprintf("%s Line %d, Column %d", s.File, s.Start.Line, s.Start.Column)
			}
			return fmt.Sprintf("%s Line %d", s.File, s.Start.Line)
		}
		return fmt.Sprintf(s.File)
	}
	if s.Start.Line > 0 {
		return fmt.Sprintf("Line %d", s.Start.Line)
	}
	return "?"
}
