// Under the MIT License.  See LICENSE file for details.

package display

import (
	"strings"
	"unicode"
)

func MakeIndent(char string, count int) string {
	r := ""
	for i := 0; i < count; i++ {
		r += char
	}
	return r
}

type SplitWith struct {
	FirstIndent      string
	SecondIndent     string
	LongLineSplitter string
	Eol              string
}

// WordSplit creates nice, even splitting of the text across the whole line.
//
// This assumes that the text will be on its own line, with only the indent before it.
// The EOL will be added to the end of the text.
func (d *Display) WordSplit(
	text string,
	w SplitWith,
) string {
	fIdt := []rune(w.FirstIndent)
	fIdtLen := len(fIdt)
	nIdt := []rune(w.SecondIndent)
	nIdtLen := len(nIdt)
	sLn := []rune(w.LongLineSplitter)
	sLnLen := len(sLn)
	if fIdtLen+sLnLen >= d.width || nIdtLen+sLnLen >= d.width {
		panic("indent with the splitter must not be longer than the display width")
	}

	s := splitData{
		// Start the first line as indented with the first indent.
		line:         fIdt,
		lineLen:      fIdtLen,
		lineHasWords: false,
		wordBreak:    []rune{},
		useWordBreak: false,
		wordBreakLen: 0,
		word:         make([]rune, 0),
		wordLen:      0,
		lines:        make([]string, 0),
		splitLine:    sLn,
		splitLineLen: sLnLen,
		fullWidth:    d.width,
		indent:       nIdt,
		indentLen:    nIdtLen,
	}

	wasEolR := false

	for _, c := range []rune(text) {
		switch {
		case c == eolN && !wasEolR:
			// \n by itself.
			s.finishLine(true)
		case c == eolN && wasEolR:
			// \r\n
			// do nothing.
		case c == eolN || c == eolR:
			s.finishLine(true)
		case unicode.IsSpace(c):
			// TODO This should include line split characters, such as splitLine string.
			s.finishWord()
			s.wordBreak = append(s.wordBreak, c)
			s.wordBreakLen++
			s.useWordBreak = true
		default:
			s.word = append(s.word, c)
			s.wordLen++
		}
		wasEolR = c == eolR
	}

	return s.close(w.Eol)
}

const eolN = rune('\n')
const eolR = rune('\r')

type splitData struct {
	line         []rune
	lineLen      int
	lineHasWords bool

	wordBreak    []rune
	useWordBreak bool
	wordBreakLen int
	word         []rune
	wordLen      int

	lines []string

	splitLine    []rune
	splitLineLen int

	fullWidth int
	indent    []rune
	indentLen int
}

func (s *splitData) clearWordBreak() {
	s.useWordBreak = false
	s.wordBreak = []rune{}
	s.wordBreakLen = 0
}

func (s *splitData) appendLineVal(line []rune) {
	s.lines = append(s.lines, string(line))
}

func (s *splitData) finishWord() {
	if s.wordLen <= 0 {
		// No word.  Do nothing.
		// Early exit here allows for word break to build up over multiple spaces.
		return
	}
	if s.lineHasWords && s.lineLen+s.wordLen+s.wordBreakLen > s.fullWidth {
		s.appendLineVal(s.line)
		s.line = make([]rune, 0)
		s.lineLen = 0
		s.clearWordBreak()
	}
	if s.useWordBreak {
		s.line = append(s.line, s.wordBreak...)
		s.lineLen += s.wordBreakLen
		s.clearWordBreak()
	}
	s.lineHasWords = true
	s.line = append(s.line, s.word...)
	s.lineLen += s.wordLen
	s.word = make([]rune, 0)
	s.wordLen = 0

	// If the unbroken word is too big for a line, force a break.
	// This assumes that the splitLineLen < width
	splitAt := s.fullWidth - s.splitLineLen
	for s.lineLen > s.fullWidth {
		// Be careful not to overwrite s.line contents.
		b1 := make([]rune, 0)
		b1 = append(b1, s.line[:splitAt]...)
		b1 = append(b1, s.splitLine...)
		s.appendLineVal(b1)
		b2 := make([]rune, 0)
		b2 = append(b2, s.indent...)
		b2 = append(b2, s.line[splitAt:]...)
		s.line = b2
		s.lineLen -= splitAt - s.indentLen
	}
}

func (s *splitData) finishLine(forceEol bool) {
	s.finishWord()
	if s.lineLen > 0 || forceEol {
		s.appendLineVal(s.line)
	}
	s.line = make([]rune, 0)
	s.lineLen = 0
	s.lineHasWords = false
}

func (s *splitData) close(eol string) string {
	s.finishLine(false)
	return strings.Join(s.lines, eol) + eol
}
