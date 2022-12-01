#!/usr/bin/env python3
import argparse
import subprocess
import re
import os

def main(args):
    data = []
    for idx in range(args.loops):
        cmd = "/usr/bin/time {}".format(" ".join(args.target))
        stdout = subprocess.getoutput(cmd)
        if m := re.search(r"(?P<user>[0123456789\.]+)user\D*(?P<sys>[0123456789\.]+)system", stdout):
            data.append((float(m.group("user")), float(m.group("sys"))))
        if idx == 0 and not args.hide: print(stdout)
    assert len(data) == args.loops
    print("user: {}".format(sum([x[0] for x in data])/args.loops))
    print("sys : {}".format(sum([x[1] for x in data])/args.loops))

if __name__=="__main__":
  parser = argparse.ArgumentParser(description='Bench a program.')
  parser.add_argument('-l', '--loops', dest='loops', type=int, default=10, help='Loops')
  parser.add_argument('-H', '--hide-output', dest='hide', action='store_true')
  parser.add_argument('target', type=str, nargs='+', help='')
      
  main(parser.parse_args())