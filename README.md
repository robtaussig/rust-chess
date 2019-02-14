# Rust Chess

## Known bug

The engine will not reply with a best move if it evaluates a forced checkmate against the side requesting the best move -- essentially, the minimax algorithm renders all moves equally undesirable. I have considered fixing this, and I will one day, although it is not a high priority -- partially because it brings me enjoyment to think of it throwing a tantrum, refusing to make a losing move.