import { spawnSync } from 'node:child_process'

const commands = [
  ['node', ['scripts/verify-build.mjs'], 'desktop P4 structural checks'],
  ['cargo', ['test', '--manifest-path', 'src-tauri/Cargo.toml'], 'Rust P4 tests'],
  ['py', ['-3', '../../tools/wiki_lint.py', '--json'], 'read-only Wiki lint'],
  ['py', ['-3', '../../tools/wiki_eval.py', '--answers-dir', '../../evals/answers'], 'Wiki 10-case contract'],
  ['py', ['-3', '../../tools/core_book_eval.py'], 'core-book Recall@5'],
  ['py', ['-3', '-m', 'unittest', 'discover', '-s', '../../tests'], 'toolchain unit tests'],
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

console.log('\nP4 verification passed.')
