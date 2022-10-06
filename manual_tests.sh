#!/usr/bin/env bash

# =================================================================================================
# start editor with decision table
# =================================================================================================

# start new session
tmux new-session -d -s Atto './target/debug/atto ./examples/e1.dtb'

sleep 0.1

#tmux send-keys -t Atto a
#tmux capture-pane -t Atto -p -S > a.txt

# Ctrl+q : close editor; session will be automatically closed
tmux send-keys -t Atto C-q

# =================================================================================================
# start editor without any arguments
# =================================================================================================

tmux new-session -d -s AttoNoArgs './target/debug/atto'

# =================================================================================================
# start editor with non-existing file
# =================================================================================================

tmux new-session -d -s AttoNonExistingFile './target/debug/atto ./examples/nop.dtb'