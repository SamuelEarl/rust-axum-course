# -q      = quiet
# -c      = clear (in between each recompile)
# -w src/ = watch the folder that is listed after the -w
# -x run  = execute the command that is listed after the -x
watch-src:
	bacon run

# Syntax:
# bacon test -- --test <filename-inside-the-tests-folder> <function-name-inside-test-file> -- --nocapture
# The `-- --nocapture` part will allow the output of println statements to pass through to the terminal.
watch-tests:
	bacon test -- --test quick_dev quick_dev -- --nocapture
