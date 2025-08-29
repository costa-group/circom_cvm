import pandas as pd
import json
import os

# --- Load metrics JSON lines ---
with open("metrics.jsonl") as f:
    data = [json.loads(line) for line in f]

# Extract just the base file name
for entry in data:
    base = os.path.basename(entry['file'])
    entry['file'] = os.path.splitext(base)[0]

df = pd.DataFrame(data)

# Total time (sum of all phases)
df['Total Time (s)'] = df[['time_read','time_parse','time_typecheck','time_cfg','time_json','time_dot']].sum(axis=1)

# Compute percentages for each phase
for col in ['time_read','time_parse','time_typecheck','time_cfg','time_json','time_dot']:
    pct_col = f'{col}_pct'
    df[pct_col] = (df[col] / df[['time_read','time_parse','time_typecheck','time_cfg','time_json','time_dot']].sum(axis=1) * 100)

# --- Load compilation times ---
comp_df = pd.read_csv("compilation_circom_cvm_times.csv")
comp_df['file'] = comp_df['file'].apply(lambda x: os.path.splitext(x)[0])  # remove extension

# Merge compilation times into metrics
df = pd.merge(df, comp_df, left_on='file', right_on='file', how='left')

# Compute percentage of total metrics time vs compilation time
df['Total time / CIRCOM to CVM (%)'] = (df['Total Time (s)'] / df['compile_time_s'] * 100).round(2)

# Rename columns for LaTeX
df = df.rename(columns={
    'file': 'Source file name',
    'num_lines': 'Lines',
    'num_cfgs': 'Nº CFGs',
    'avg_blocks_per_cfg': 'Avg. Blocks/CFG',
    'avg_variables_per_cfg': 'Avg. Variables/CFG',
    'avg_stmts_per_block': 'Avg. Stmts/Block',
    'time_read_pct': 'Read (%)',
    'time_parse_pct': 'Parse (%)',
    'time_typecheck_pct': 'Typecheck (%)',
    'time_cfg_pct': 'CFG (%)',
    'time_json_pct': 'JSON Write (%)',
    'time_dot_pct': 'DOT Write (%)',
    'compile_time_s': 'CIRCOM to CVM (s)'
})

# Columns to round to 2 decimals
two_decimal_cols = [
    'Lines', 'Nº CFGs', 'Avg. Blocks/CFG', 'Avg. Variables/CFG', 'Avg. Stmts/Block',
    'Read (%)', 'Parse (%)', 'Typecheck (%)', 'CFG (%)', 'JSON Write (%)', 'DOT Write (%)',
    'Total time / CIRCOM to CVM (%)'
]

df[two_decimal_cols] = df[two_decimal_cols].round(2)

# Format total time and compilation time with full precision
df['Total Time (s)'] = df['Total Time (s)'].apply(lambda x: f"{x:.6f}")
df['CIRCOM to CVM (s)'] = df['CIRCOM to CVM (s)'].apply(lambda x: f"{x:.6f}")

# Select columns for LaTeX export
columns_to_export = [
    'Source file name', 'Lines', 'Nº CFGs', 'Avg. Blocks/CFG', 'Avg. Variables/CFG',
    'Avg. Stmts/Block', 'Read (%)', 'Parse (%)', 'Typecheck (%)', 'CFG (%)',
    'JSON Write (%)', 'DOT Write (%)', 'Total Time (s)', 'CIRCOM to CVM (s)',
    'Total time / CIRCOM to CVM (%)'
]

# Sort by total time ascending
df = df.sort_values(by='Total Time (s)', ascending=True)

# Export to LaTeX
with open("metrics_table.tex", "w") as out:
    out.write(df[columns_to_export].to_latex(index=False, escape=True))

print("✅ LaTeX table generated as metrics_table.tex")
