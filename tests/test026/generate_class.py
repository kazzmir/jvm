"""Generate a Java 5 class using jsr/ret, which modern javac cannot emit.

Equivalent behavior: initialize a local to zero, increment it in a subroutine
called twice, then print it. Both JVMs execute this exact generated class.
"""
from pathlib import Path
import struct


def u2(value):
    return struct.pack('>H', value)


def u4(value):
    return struct.pack('>I', value)


def utf8(text):
    data = text.encode('utf-8')
    return b'\x01' + u2(len(data)) + data


pool = [
    utf8('Main'),                              # 1
    b'\x07' + u2(1),                           # 2: Class Main
    utf8('java/lang/Object'),                  # 3
    b'\x07' + u2(3),                           # 4: Class Object
    utf8('main'),                              # 5
    utf8('([Ljava/lang/String;)V'),            # 6
    utf8('Code'),                              # 7
    utf8('java/lang/System'),                  # 8
    b'\x07' + u2(8),                           # 9: Class System
    utf8('out'),                               # 10
    utf8('Ljava/io/PrintStream;'),             # 11
    b'\x0c' + u2(10) + u2(11),                 # 12: NameAndType out
    b'\x09' + u2(9) + u2(12),                  # 13: Fieldref System.out
    utf8('java/io/PrintStream'),               # 14
    b'\x07' + u2(14),                          # 15: Class PrintStream
    utf8('println'),                           # 16
    utf8('(I)V'),                              # 17
    b'\x0c' + u2(16) + u2(17),                 # 18: NameAndType println
    b'\x0a' + u2(15) + u2(18),                 # 19: Methodref println
]

code = bytes([
    0x03,                    # 0: iconst_0
    0x3c,                    # 1: istore_1
    0xa8, 0x00, 0x0e,        # 2: jsr 16 (return to 5)
    0xa8, 0x00, 0x0b,        # 5: jsr 16 (return to 8)
    0xb2, 0x00, 0x0d,        # 8: getstatic System.out
    0x1b,                    # 11: iload_1
    0xb6, 0x00, 0x13,        # 12: invokevirtual println(int)
    0xb1,                    # 15: return
    0x4d,                    # 16: astore_2 (returnAddress)
    0x84, 0x01, 0x01,        # 17: iinc 1, 1
    0xa9, 0x02,              # 20: ret 2
])
code_attribute = u2(2) + u2(3) + u4(len(code)) + code + u2(0) + u2(0)
main_method = u2(0x0009) + u2(5) + u2(6) + u2(1)
main_method += u2(7) + u4(len(code_attribute)) + code_attribute
class_file = bytes.fromhex('cafebabe') + u2(0) + u2(49)
class_file += u2(len(pool) + 1) + b''.join(pool)
class_file += u2(0x0021) + u2(2) + u2(4) + u2(0) + u2(0)
class_file += u2(1) + main_method + u2(0)
Path(__file__).with_name('Main.class').write_bytes(class_file)
