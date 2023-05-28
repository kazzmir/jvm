#!/usr/bin/env python3

import os
import subprocess

def compile_java_files(path):
    def is_java_file(filename):
        return filename.endswith('.java')
    java_files = [f for f in os.listdir(path) if is_java_file(f)]
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

def do_test(path):
    """`path` should contain a set of java files.
        1. javac to compile them (if the .class files don't already exist)
        2. run `jvm` on the class files to get the actual output
        3. run `java` on the class files to get the expected output
        4. compare expected output to actual
    """

    compile_java_files(path)
    actual = run_jvm(path)
    expected = run_java(path)

    if actual != expected:
        print(f"Failure {path}: actual={actual} expected={expected}")
    else:
        print("OK")

def main():
    for path in os.listdir('tests'):
        full = os.path.join('tests', path)
        if os.path.isdir(full):
            do_test(full)

main()
