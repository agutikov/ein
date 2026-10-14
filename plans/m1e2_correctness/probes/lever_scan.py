import json, os, subprocess, sys, tempfile
ROOT=os.path.abspath(os.path.join(os.path.dirname(__file__), "../../..")); EIN=os.path.join(ROOT,"ein.rs/target/release/ein")
scan=json.load(open(sys.argv[1]))
def run(path, extra):
    with tempfile.TemporaryDirectory() as td:
        js=os.path.join(td,"s.json")
        try: subprocess.run([EIN,"solve","-e",*extra,path,"--json-summary",js],cwd=ROOT,capture_output=True,text=True,timeout=60)
        except subprocess.TimeoutExpired: return None
        if not os.path.exists(js): return None
        v=json.load(open(js))["verdict"]; return (v["type"],v["k"],v["exhausted"])
out=[]
for rec in scan:
    if rec.get("error"): continue
    p=rec["path"]; d=run(p,[]); L=run(p,["-L"]); K=run(p,["-K"])
    row={"path":p,"default":d,"L":L,"K":K}; out.append(row)
    if d and L and (d[0],d[1])!=(L[0],L[1]) or d and K and (d[0],d[1])!=(K[0],K[1]):
        print(p,"default",d,"-L",L,"-K",K,flush=True)
json.dump(out,open(sys.argv[2],"w"),indent=1)
