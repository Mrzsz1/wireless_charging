import { spawnSync } from 'node:child_process'

const commands = [
  ['node', ['scripts/verify-build.mjs'], 'desktop structural checks'],
  ['cargo', ['test', '--manifest-path', 'src-tauri/Cargo.toml'], 'Rust P3 tests'],
  ['py', ['-3', '../../tools/wiki_eval.py', '--answers-dir', '../../evals/answers'], 'Wiki 10-case contract'],
  ['py', ['-3', '../../tools/core_book_eval.py'], 'core-book Recall@5'],
]

for (const [command, args, label] of commands) {
  console.log(`\n=== ${label} ===`)
  const result = spawnSync(command, args, { cwd: process.cwd(), stdio: 'inherit', shell: false })
  if (result.error) {
    console.error(`${label} failed to start: ${result.error.message}`)
    process.exit(1)
  }
  if (result.status !== 0) process.exit(result.status ?? 1)
}

console.log('\nP3 verification passed.')
