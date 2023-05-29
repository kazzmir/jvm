#!/usr/bin/env python3

import os
import subprocess
import sys
import stat

def last_modification(path):
    statobj = os.stat(path)
    return statobj[stat.ST_MTIME]

def replace_extension(path, new):
    return os.path.splitext(path)[0] + new

def compile_java_files(path):
    def is_java_file(filename):
        return filename.endswith('.java')
    java_files = [f for f in os.listdir(path) if is_java_file(f)]

    need_compile = False
    for java in java_files:
        class_file = os.path.join(path, replace_extension(java, '.class'))
        if not os.path.exists(class_file) or last_modification(class_file) < last_modification(os.path.join(path, java)):
            need_compile = True
            break

    if need_compile:
        subprocess.call(['javac'] + java_files, cwd=path)

def run_jvm(path):
    process = subprocess.run(['./jvm', os.path.join(path, 'Main.class')], capture_output=True)
    if process.returncode != 0:
        return b'failed to run: ' + process.stderr
    return process.stdout

def run_java(path):
    process = subprocess.run(['java', '-classpath', path, 'Main'], capture_output=True)
    if process.returncode != 0:
        return b'java failed to run: ' + process.stderr
    return process.stdout

def xterm_color_start(color):
    return '\033[%sm' % color

def xterm_color_end():
    return '\033[0m'

def colorize(s, color):
    colors = {
        'green': '0;32',
        'red': '0;31',
    }

    return xterm_color_start(colors[color]) + s + xterm_color_end()

def do_test(path):
    """`path` should contain a set of java files.
        1. javac to compile them (if the .class files don't already exist)
        2. run `jvm` on the class files to get the actual output
        3. run `java` on the class files to get the expected output
        4. compare expected output to actual
    """

    print(path + ': ', end='')
    sys.stdout.flush()

    compile_java_files(path)
    actual = run_jvm(path)
    expected = run_java(path)

    if actual != expected:
        print(f"{colorize('Failure', 'red')} actual={actual} expected={expected}")
    else:
        print(colorize("OK", 'green'))

def main():
    for path in os.listdir('tests'):
        full = os.path.join('tests', path)
        if os.path.isdir(full):
            do_test(full)

main()
