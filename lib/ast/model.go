// Under the MIT License.  See LICENSE file for details.

package ast

// Tree structure represents the data processing path.
//
// A command may have many data processing paths.
type Tree struct {
	Name     string
	Actions  []TreeAction
	Monitors []Monitor
}

// TreeAction is exactly one of Execution, StreamFilter, Tree, or WaitExecution.
type TreeAction struct {
	*Execution
	*StreamFilter
	*WaitExecution
	*Tree
}

// DataStream represents data moving through the command.
//
// Data streams have filters and extra data added into them.  Conceptually, each
// manipulation of the data constructs a new stream, but the data processing references
// it through the high level DataStream.
type DataStream string

// Execution structure represents the execution of code or another process.
type Execution struct {
	Source

	Template   TemplateName // Template name for the handler code.
	Parameters              // Parameter values to the general handler code.

	Inputs  []DataStream           // Data streams input into the execution, ordered by FD.
	Outputs []DataStream           // Data streams output from the execution, ordered by FD.
	Exits   []ExecutionExitHandler // How to handle the exit code.
}

// ExecutionExitHandler describes what to do under a specific execution exit condition.
type ExecutionExitHandler struct {
	Source
	Template TemplateName // Template name for the handler code.
}

// StreamFilter references a template filter with parameters.
//
// The filter is applied on top of a stream.
type StreamFilter struct {
	Source
	WithTemplate            // The template used to handle the filtering.
	Input        DataStream // Input data to the filter.
	Output       DataStream // The name of the output from the filter.
}

// Monitor continuously runs until some event occurs.
type Monitor struct {
	Source
	WithTemplate
	ExitSignal Signal
	Triggers   Signal
}

// WaitExecution acts like an execution, buts waits for a signal to trigger or a timeout.
type WaitExecution struct {
	Source
	Signal
	TimeoutSeconts float64
}

// WithTemplate describes the template and general parameters for using a template.
type WithTemplate struct {
	TemplateName
	Parameters
}

// TemplateName references code that generates into the script to run.
type TemplateName string

// Parameters maintain a relationship between a distinct name and a possible value type.
type Parameters map[string]ParameterValue

// ParameterValue contains the typed value of the parameter.
//
// The value is an interface for an int64, float64, boolean, or string.
type ParameterValue interface{}

// Signal represents a named source for alerting between processes within the script.
type Signal string

// Source marks the full position for the source of a piece of an abstract syntax tree.
type Source struct {
	File  string
	Start SourceFilePosition
	End   SourceFilePosition
}

// SourceFilePosition marks the line number and character column for a position in a text file.
type SourceFilePosition struct {
	Line   int
	Column int
}
