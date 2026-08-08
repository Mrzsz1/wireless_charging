export type LatestRequestGuard = {
  next(): number
  isCurrent(token: number): boolean
  invalidate(): number
}

export function createLatestRequestGuard(): LatestRequestGuard {
  let generation = 0
  return {
    next() {
      generation += 1
      return generation
    },
    isCurrent(token) {
      return token === generation
    },
    invalidate() {
      generation += 1
      return generation
    },
  }
}
