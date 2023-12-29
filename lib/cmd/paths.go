// Under the MIT License.  See LICENSE file for details.

package cmd

import (
	"os"
	"path/filepath"
)

const EnvLibDirKey = "NATIVE_SHELL_LIBS"
const EnvHomeDirKey = "NATIVE_SHELL_HOME"
const DefaultShellHome = ".local/share/native-shell"
const ShellHomeLibDir = "lib"

func DefaultLibDir(env map[string]string) (string, error) {
	if libs, ok := env[EnvLibDirKey]; ok {
		return libs, nil
	}
	home, err := DefaultHomeDir(env)
	if err != nil {
		return "", err
	}
	return filepath.Join(home, ShellHomeLibDir), nil
}

func DefaultHomeDir(env map[string]string) (string, error) {
	if home, ok := env[EnvHomeDirKey]; ok {
		return home, nil
	}
	path, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(path, DefaultShellHome), nil
}
