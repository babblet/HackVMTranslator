import subprocess
import sys
import os

if os.path.isdir(sys.argv[1]):
    in_path = sys.argv[1]
    out_path = sys.argv[1] + '/' + sys.argv[1].split('/')[-1] + ".asm"
    print(out_path)
    subprocess.call(['./hack_vm_translator', in_path, out_path])
else:
    subprocess.call(['./hack_vm_translator', sys.argv[1], sys.argv[1].replace("vm", "asm")])
    print(sys.argv[1])
    
    
