#!/usr/bin/env bash

# =================================================================================================
# start editor with decision table
# =================================================================================================

# start new session
tmux new-session -d -s Atto './target/debug/atto ./examples/e1.dtb'

# wait until the editor starts
sleep 0.2

# send unhandled key combinations to make debug message to be displayed
tmux send-keys -t Atto C-M-End
#tmux capture-pane -t Atto -p -S

# Ctrl+q : close editor; session will be automatically closed
tmux send-keys -t Atto C-q

# =================================================================================================
# start editor without any arguments
# =================================================================================================

tmux new-session -d -s AttoNoArgs './target/debug/atto'

sleep 0.2

# =================================================================================================
# start editor with non-existing file
# =================================================================================================

tmux new-session -d -s AttoNonExistingFile './target/debug/atto ./examples/nop.dtb'

sleep 0.2