import subprocess
import sys

subprocess.call(['./hack_vm_translator', sys.argv[1], sys.argv[1].replace("vm", "asm")])
