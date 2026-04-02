#!/usr/bin/env python3
"""Fix lifetime annotations in module files."""

import re
import os
import glob

def fix_file(filepath):
    """Add lifetime parameter to function signatures that use 'a but don't declare it."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Find functions that have <'a> in parameters but not in signature
    # Pattern: pub fn name(\n    args: &[Expr<'a>]
    pattern = r'(pub fn [a-z_]+)\(\s*\n\s*args: &\[Expr<\'a>\]'
    
    def replace_func(match):
        func_name = match.group(1)
        # Check if function already has lifetime parameter
        if "<'a>" not in func_name:
            return f"{func_name}<'a>(\n    args: &[Expr<'a>]"
        return match.group(0)
    
    new_content = re.sub(pattern, replace_func, content)
    
    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Fixed: {filepath}")
        return True
    return False

# Find all module files
module_files = glob.glob("crates/rydit-rs/src/modules/*.rs")

fixed_count = 0
for filepath in module_files:
    if fix_file(filepath):
        fixed_count += 1

print(f"\nFixed {fixed_count} files")
