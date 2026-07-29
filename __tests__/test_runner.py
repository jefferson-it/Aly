# __tests__/test_runner.py
import os
import subprocess
import sys

def main():
    # Find all pairs of .aly and .py files in __tests__
    tests_dir = os.path.dirname(os.path.abspath(__file__))
    
    files = os.listdir(tests_dir)
    test_pairs = []
    for f in files:
        if f.endswith('.aly'):
            base = f[:-4]
            if "showcase" in base:
                continue
            py_partner = base + '.py'
            if py_partner in files:
                test_pairs.append(base)
                
    test_pairs.sort()
    
    if not test_pairs:
        print("No test pairs (.aly and .py) found in __tests__.")
        return

    print("=== Aly vs Python Output Comparator ===")
    print("Available tests:")
    for idx, name in enumerate(test_pairs):
        print(f"[{idx + 1}] {name}")
    print(f"[{len(test_pairs) + 1}] Run All Tests")
    print("[0] Exit")

    try:
        choice = input("\nSelect a test to execute: ").strip()
        if not choice:
            return
        
        choice_idx = int(choice)
        if choice_idx == 0:
            print("Exiting.")
            return
        
        if choice_idx == len(test_pairs) + 1:
            run_all(test_pairs, tests_dir)
        elif 1 <= choice_idx <= len(test_pairs):
            selected = test_pairs[choice_idx - 1]
            run_comparison(selected, tests_dir)
        else:
            print("Invalid choice.")
    except ValueError:
        print("Please enter a valid number.")

def run_comparison(name, tests_dir):
    aly_file = os.path.join(tests_dir, f"{name}.aly")
    py_file = os.path.join(tests_dir, f"{name}.py")
    
    # Try finding aly binary in target/release or target/debug
    aly_bin = os.path.join(tests_dir, "..", "target", "release", "aly")
    if not os.path.exists(aly_bin):
        aly_bin = os.path.join(tests_dir, "..", "target", "debug", "aly")
    if not os.path.exists(aly_bin):
        aly_cmd = ["cargo", "run", "--quiet", "--bin", "aly", "--", "run", aly_file]
    else:
        aly_cmd = [aly_bin, "run", aly_file]
        
    print(f"\nRunning {name}.aly ...")
    try:
        aly_result = subprocess.run(aly_cmd, capture_output=True, text=True, check=True)
        aly_output = aly_result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running Aly interpreter: {e.stderr}")
        aly_output = None
        
    print(f"Running {name}.py ...")
    try:
        py_cmd = [sys.executable, py_file]
        py_result = subprocess.run(py_cmd, capture_output=True, text=True, check=True)
        py_output = py_result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running Python: {e.stderr}")
        py_output = None

    if aly_output is None or py_output is None:
        print("Comparison aborted due to execution error.")
        return False

    print("\n--- Output Comparison ---")
    if aly_output == py_output:
        print("✅ SUCCESS: Output matches exactly!")
        print("\nMatched Output:")
        print(aly_output)
        return True
    else:
        print("❌ FAIL: Output mismatch!")
        print("\nAly Output:")
        print(repr(aly_output))
        print("\nPython Output:")
        print(repr(py_output))
        return False

def run_all(test_pairs, tests_dir):
    print("\nRunning all comparisons:")
    success = 0
    for name in test_pairs:
        print(f"\n==================== {name} ====================")
        if run_comparison(name, tests_dir):
            success += 1
            
    print(f"\nSummary: {success}/{len(test_pairs)} tests passed.")

if __name__ == '__main__':
    main()
