import os
import subprocess

def main():
    test_directory = './tests/json_org_tests/'
    pass_count = 0
    fail_count = 0

    # Get all test files in the directory
    test_files = [f for f in os.listdir(test_directory) if f.endswith('.json')]
    
    for test_file in test_files:
        test_file_path = os.path.join(test_directory, test_file)
        expected_result = "PASS" if test_file.startswith("pass") else "FAIL"
        
        # Run the Rust program with the --release flag
        result = subprocess.run(
            ['cargo', 'run', '--release', test_file_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding='utf-8'
        )

        if result.stdout:
            output = result.stdout.splitlines()
            first_line = output[0].strip()
            if first_line == expected_result:
                pass_count += 1
            else:
                fail_count += 1
                print(f"Unexpected output in {test_file}: {first_line}")
        else:
            fail_count += 1
            print(f"No output from {test_file}")
        
    # Print the summary
    print(f"Total tests: {len(test_files)}")
    print(f"Passed: {pass_count}")
    print(f"Failed: {fail_count}")

if __name__ == "__main__":
    main()
